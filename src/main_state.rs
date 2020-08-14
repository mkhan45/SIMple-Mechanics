use ggez::event::EventHandler;
use ggez::{
    graphics,
    input::{
        keyboard::{KeyCode, KeyMods},
        mouse::MouseButton,
    },
};

use specs::prelude::*;

use crate::{
    BodySet, Collider, ColliderSet, ForceGeneratorSet, GeometricalWorld, JointConstraintSet,
    MechanicalWorld, Vector,
};
use crate::{SCREEN_X, SCREEN_Y};

use crate::components::*;

use crate::resources::{
    self, CreateElasticity, CreateFriction, CreateMass, CreateShapeStatic, FrameSteps, MousePos,
    Paused, ShapeInfo,
};

use crate::gui::imgui_wrapper::{ImGuiWrapper, UiChoice};

use ncollide2d as nc;
use nphysics2d as np;
use resources::{
    CreateShapeCentered, CreationData, GraphPosData, HiDPIFactor, MouseStartPos, MovingGraph,
    ScalingGraph,
};

pub mod body_builder;
use body_builder::BodyBuilder;

mod util;
use util::*;

pub struct MainState<'a, 'b> {
    pub world: specs::World,
    pub dispatcher: Dispatcher<'a, 'b>,
    pub imgui_wrapper: ImGuiWrapper,
}

impl<'a, 'b> MainState<'a, 'b> {
    pub fn delete_entity(&mut self, entity: Entity) {
        // to delete an entity, it needs to be removed
        // from the nphysics body and collider sets
        // before being removed from the specs world.
        // NEVER call world.delete_entity() to remove
        // a physics object.
        {
            let mut body_set = self.world.fetch_mut::<BodySet>();
            let body_storage = self.world.read_storage::<PhysicsBody>();
            let body_handle = body_storage.get(entity).unwrap();

            let mut collider_set = self.world.fetch_mut::<ColliderSet>();
            let collider_storage = self.world.read_storage::<Collider>();
            let collider_handle = collider_storage.get(entity).unwrap();

            body_set.remove(body_handle.body_handle);
            collider_set.remove(collider_handle.coll_handle);
        }

        self.imgui_wrapper.remove_sidemenu();
        self.world.delete_entity(entity).unwrap();
    }

    pub fn delete_all(&mut self) {
        let mut delete_buff: Vec<Entity> = Vec::new();

        {
            let physics_bodies = self.world.read_storage::<PhysicsBody>();
            let entities = self.world.entities();

            (&physics_bodies, &entities).join().for_each(|(_, entity)| {
                delete_buff.push(entity);
            });
        }

        delete_buff.iter().for_each(|entity| {
            self.delete_entity(*entity);
        });
    }

    pub fn reactivate_all(&mut self) {
        let bodies = self.world.read_storage::<PhysicsBody>();
        let mut body_set = self.world.fetch_mut::<BodySet>();

        bodies.join().for_each(|body| {
            body_set.get_mut(body.body_handle).unwrap().activate();
        });
    }
}

impl<'a, 'b> EventHandler for MainState<'a, 'b> {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        {
            self.world.insert(resources::DT(ggez::timer::delta(ctx)));
            self.world.insert(resources::FPS(ggez::timer::fps(ctx)));

            self.world.maintain();

            self.process_gui_signals();
            self.lua_update();

            self.dispatcher.dispatch(&self.world);
        }

        {
            // only one physics body should have the InfoDisplayed component;
            // maybe it should be a resource: TODO
            let info_displayed = self.world.read_storage::<InfoDisplayed>();
            let entities = self.world.entities();
            if let Some((_, entity)) = (&info_displayed, &entities).join().next() {
                self.imgui_wrapper.remove_sidemenu();
                self.imgui_wrapper
                    .shown_menus
                    .insert(UiChoice::SideMenu(entity));
            }
        }

        // run the physics step
        let geometrical_world = &mut self.world.fetch_mut::<GeometricalWorld>();
        let body_set = &mut *self.world.fetch_mut::<BodySet>();
        let collider_set = &mut *self.world.fetch_mut::<ColliderSet>();
        let joint_constraint_set = &mut *self.world.fetch_mut::<JointConstraintSet>();
        let force_generator_set = &mut *self.world.fetch_mut::<ForceGeneratorSet>();
        let mut mechanical_world = self.world.fetch_mut::<MechanicalWorld>();

        // not running the physics step at all when paused causes some weird behavior,
        // so just run it with a timestep of 0
        if self.world.fetch::<Paused>().0 {
            mechanical_world.set_timestep(0.0);
        } else {
            mechanical_world.set_timestep(self.world.fetch::<resources::Timestep>().0);
        }

        (0..self.world.fetch::<FrameSteps>().0).for_each(|_| {
            mechanical_world.step(
                geometrical_world,
                body_set,
                collider_set,
                joint_constraint_set,
                force_generator_set,
            );
        });

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        use graphics::Color;

        graphics::clear(ctx, graphics::BLACK);

        // the mesh builder batches all of the objects into a single mesh,
        // reducing draw calls.
        // It's not used for graphs or the GUI
        let mut mesh_builder = graphics::MeshBuilder::new();

        {
            let entities = self.world.entities();

            let colliders = self.world.read_storage::<Collider>();
            let colors = self.world.read_storage::<crate::components::Color>();
            let collider_set = self.world.fetch::<ColliderSet>();

            let selected = self.world.fetch::<resources::Selected>().0;

            // worth noting that only the collider is drawn, not the
            // actual physics objects, which don't inherently have shape.
            //
            // Pseudocode:
            // iterate through each collider and its associated color.
            // in each iteration:
            //      1. Get its position and rotation
            //      2. Cast its generic shape to either a Ball or a Rect
            //      3. Draw the actual shape
            //      4. If it has the Selected component, draw a red bounding box around it
            (&colliders, &colors, &entities)
                .join()
                .for_each(|(collider_comp, color, e)| {
                    let collider = collider_set
                        .get(collider_comp.coll_handle)
                        .expect("error getting collider to draw");

                    let (pos, rot) = {
                        let isometry = collider.position();
                        let na_vector = isometry.translation.vector;
                        ([na_vector.x, na_vector.y], isometry.rotation.angle())
                    };

                    if collider.shape().is_shape::<nc::shape::Ball<f32>>() {
                        let shape = collider
                            .shape()
                            .downcast_ref::<nc::shape::Ball<f32>>()
                            .expect("bad shape");

                        draw_circle(&mut mesh_builder, pos, rot, shape.radius(), color.0, false);

                        if let Some(selected) = selected {
                            if selected == e {
                                draw_circle(
                                    &mut mesh_builder,
                                    pos,
                                    rot,
                                    shape.radius(),
                                    Color::new(1.0, 0.0, 0.0, 1.0),
                                    true,
                                );
                            }
                        }
                    } else if collider.shape().is_shape::<nc::shape::Cuboid<f32>>() {
                        let shape = collider
                            .shape()
                            .downcast_ref::<nc::shape::Cuboid<f32>>()
                            .expect("bad shape");

                        draw_rect(
                            &mut mesh_builder,
                            pos,
                            rot,
                            *shape.half_extents(),
                            color.0,
                            false,
                        );

                        if let Some(selected) = selected {
                            if selected == e {
                                draw_rect(
                                    &mut mesh_builder,
                                    pos,
                                    rot,
                                    *shape.half_extents(),
                                    graphics::Color::new(1.0, 0.0, 0.0, 1.0),
                                    true,
                                );
                            }
                        }
                    }
                });

            // draws the outlined circle and rect if you're creating an object
            self.draw_creation_gui(&mut mesh_builder);
            if let Ok(mesh) = mesh_builder.build(ctx) {
                // this _will_ error if there's an empty screen, so just ignore it.
                // Even if the error is something else it's pretty easy to tell
                // when something goes wrong here because no shapes will be drawn
                let _ = graphics::draw(ctx, &mesh, graphics::DrawParam::new());
            }
        }

        {
            // draw the graph and labels for the midpoint, top, and bottom

            let (graph_text, graph_builder) = self.draw_graphs();
            if let Ok(mesh) = graph_builder.build(ctx) {
                let graph_rect = self.world.fetch::<GraphPosData>().0;
                let scale_fac = graph_rect.w / 10.0;
                let _ = graphics::draw(
                    ctx,
                    &mesh,
                    graphics::DrawParam::new()
                        .dest([graph_rect.x, graph_rect.y])
                        .scale([scale_fac, scale_fac]),
                );

                // redundant since we just iterate later
                // but otherwise it's easy to forget the order
                let [max_text, mid_text, min_text] = graph_text;
                let y_translations = [0.25, 5.0, 9.25];

                [max_text, mid_text, min_text]
                    .iter()
                    .zip(y_translations.iter())
                    .for_each(|(text, y_translation)| {
                        let _ = graphics::draw(
                            ctx,
                            text,
                            graphics::DrawParam::new()
                                .dest([
                                    graph_rect.x + 0.25 * scale_fac,
                                    graph_rect.y + y_translation * scale_fac,
                                ])
                                .scale([0.025 * scale_fac, 0.025 * scale_fac]),
                        );
                    });
            }
        }

        {
            let hidpi_factor = self.world.fetch::<HiDPIFactor>().0;
            self.imgui_wrapper
                .render(ctx, hidpi_factor, &mut self.world);
        }

        graphics::present(ctx)?;

        Ok(())
    }

    fn resize_event(&mut self, ctx: &mut ggez::Context, width: f32, height: f32) {
        // making width increase with respect to the height reveals more things
        // making the height increase with respect to the width scales everything down

        let aspect_ratio = height / width;
        let initial_ratio = 1.0;

        if initial_ratio > aspect_ratio {
            let new_width = SCREEN_X / aspect_ratio;
            ggez::graphics::set_screen_coordinates(
                ctx,
                ggez::graphics::Rect::new(0.0, 0.0, new_width, SCREEN_Y),
            )
            .expect("error resizing");
        } else {
            let new_height = SCREEN_Y * aspect_ratio;
            ggez::graphics::set_screen_coordinates(
                ctx,
                ggez::graphics::Rect::new(0.0, 0.0, SCREEN_X, new_height),
            )
            .expect("error resizing");
        }

        self.world
            .insert(resources::Resolution(Vector::new(width, height)));

        let screen_coords = graphics::screen_coordinates(ctx);
        self.world.insert(resources::ScaleFac(Vector::new(
            screen_coords.w / SCREEN_X,
            screen_coords.h / SCREEN_Y,
        )));
    }

    fn mouse_motion_event(&mut self, ctx: &mut ggez::Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        // input mouse position data to specs world and imgui
        self.imgui_wrapper.update_mouse_pos(x, y);

        let screen_size = graphics::drawable_size(ctx);
        let screen_coords = graphics::screen_coordinates(ctx);
        let mouse_point = Vector::new(
            x / screen_size.0 * screen_coords.w,
            y / screen_size.1 * screen_coords.h,
        );

        self.world.insert(resources::MousePos(mouse_point));

        // unfinished Polyline stuff
        // {
        //     let mut create_shape_data = self.world.fetch_mut::<CreationData>();
        //     if input::mouse::button_pressed(ctx, MouseButton::Left)
        //         && ggez::timer::ticks(ctx) % 10 == 0
        //     {
        //         if let Some(ShapeInfo::Polyline(Some(points))) = create_shape_data.0.as_mut() {
        //             let mouse_pos = self.world.fetch::<resources::MousePos>().0;
        //             points.push(Point::new(mouse_pos.x, mouse_pos.y));
        //         }
        //     }
        // }
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut ggez::Context,
        btn: MouseButton,
        x: f32,
        y: f32,
    ) {
        // 1. update imgui mouse data
        // 2. Add mouse click pos as MouseStartPos to specs world (it's removed in
        //    mouse_button_up_event)

        self.imgui_wrapper.update_mouse_down((
            btn == MouseButton::Left,
            btn == MouseButton::Right,
            btn == MouseButton::Middle,
        ));

        let screen_size = graphics::drawable_size(ctx);
        let screen_coords = graphics::screen_coordinates(ctx);
        let mouse_point = Vector::new(
            x / screen_size.0 * screen_coords.w,
            y / screen_size.1 * screen_coords.h,
        );
        self.world
            .insert(resources::MouseStartPos(Some(mouse_point)));

        match btn {
            MouseButton::Left => {
                {
                    // If a left click overlaps the grab point of the graph, set MovingGraph to true.
                    // MovingGraph is used in GraphTransformSys
                    let mouse_rect = graphics::Rect::new(mouse_point.x, mouse_point.y, 0.1, 0.1);
                    let graph_grab_rect = self.graph_grab_rect();

                    if mouse_rect.overlaps(&graph_grab_rect) {
                        self.world.insert(MovingGraph(true));
                        return;
                    }
                }

                // if left click overlaps a shape, set the entity to be Selected
                if let Some(entity) = get_hovered_shape(&self.world) {
                    self.world.insert(resources::Selected(Some(entity)));
                }

                {
                    // unfinished polyline creation stuff
                    // let mut create_shape_data = self.world.fetch_mut::<CreationData>();
                    // if let Some(ShapeInfo::Polygon(Some(points))) = create_shape_data.0.as_mut() {
                    //     points.push(mouse_point.into());
                    // }
                }
            }
            MouseButton::Right => {
                {
                    // if a right click overlaps the graph grab point, set ScalingGraph to true
                    // this is used by GraphTransformSys
                    let mouse_rect = graphics::Rect::new(mouse_point.x, mouse_point.y, 0.1, 0.1);
                    let graph_grab_rect = self.graph_grab_rect();

                    if mouse_rect.overlaps(&graph_grab_rect) {
                        self.world.insert(ScalingGraph(true));
                        return;
                    }
                }

                {
                    // If a sidepanel'd object is right clicked, remove sidepanel
                    // if a non sidepanel'd object is right clicked, add sidepanel
                    let mut info_displayed = self.world.write_storage::<InfoDisplayed>();
                    match get_hovered_shape(&self.world) {
                        Some(entity) => {
                            if info_displayed.get(entity).is_some() {
                                info_displayed.remove(entity).unwrap();
                                self.imgui_wrapper.remove_sidemenu();
                            } else {
                                info_displayed
                                    .insert(entity, InfoDisplayed::default())
                                    .unwrap();
                            }
                        }
                        None => {
                            info_displayed.clear();

                            // remove all the sidemenus in shown menus
                            self.imgui_wrapper
                                .shown_menus
                                .retain(|menu| !matches!(menu, UiChoice::SideMenu(_)));
                        }
                    }
                }

                // nonworking polygon stuff
                // {
                //     let create_shape_data = self.world.fetch::<CreationData>();
                //     if let Some(ShapeInfo::Polygon(Some(_points))) = &create_shape_data.0.clone() {
                //         let start_pos = self.world.fetch::<MouseStartPos>().0.unwrap();
                //         BodyBuilder {
                //             translation: start_pos,
                //             rotation: 0.0,
                //             restitution: self.world.fetch::<CreateElasticity>().0,
                //             friction: self.world.fetch::<CreateFriction>().0,
                //             ..BodyBuilder::from_world(
                //                 &self.world,
                //                 create_shape_data.0.as_ref().unwrap().clone(),
                //                 self.world.fetch::<CreateMass>().0,
                //             )
                //         }
                //         .create();
                //     }
                // }
            }
            _ => {}
        }
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut ggez::Context,
        btn: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.imgui_wrapper.update_mouse_down((false, false, false));
        match btn {
            MouseButton::Left => {
                // unselect object and graph
                // finish creation of object
                self.world.insert(resources::Selected(None));

                self.world.insert(MovingGraph(false));

                let create_shape_opt = self.world.fetch::<CreationData>();
                let create_shape_data = create_shape_opt.0.as_ref();
                let create_shape_centered = self.world.fetch::<CreateShapeCentered>().0;
                let status = if self.world.fetch::<CreateShapeStatic>().0 {
                    np::object::BodyStatus::Static
                } else {
                    np::object::BodyStatus::Dynamic
                };

                if let Some(data) = &create_shape_data {
                    let start_pos = self.world.fetch::<MouseStartPos>().0.unwrap();
                    let current_pos = self.world.fetch::<MousePos>().0;
                    let mouse_drag_vec = start_pos - current_pos;
                    match (data, create_shape_centered) {
                        (ShapeInfo::Rectangle(_), true) => {
                            BodyBuilder {
                                translation: start_pos,
                                rotation: 0.0,
                                restitution: self.world.fetch::<CreateElasticity>().0,
                                friction: self.world.fetch::<CreateFriction>().0,
                                status,
                                ..BodyBuilder::from_world(
                                    &self.world,
                                    ShapeInfo::Rectangle(Some(mouse_drag_vec.abs())),
                                    self.world.fetch::<CreateMass>().0,
                                )
                            }
                            .create();
                            std::mem::drop(create_shape_opt);
                            self.world.insert(CreationData(None));
                        }
                        (ShapeInfo::Rectangle(_), false) => {
                            let mut start_pos = start_pos;
                            if mouse_drag_vec.x < 0.0 {
                                start_pos.x += mouse_drag_vec.x.abs();
                            }
                            if mouse_drag_vec.y < 0.0 {
                                start_pos.y += mouse_drag_vec.y.abs();
                            }

                            BodyBuilder {
                                translation: start_pos - mouse_drag_vec.abs() / 2.0,
                                rotation: 0.0,
                                restitution: self.world.fetch::<CreateElasticity>().0,
                                friction: self.world.fetch::<CreateFriction>().0,
                                status,
                                ..BodyBuilder::from_world(
                                    &self.world,
                                    ShapeInfo::Rectangle(Some(mouse_drag_vec.abs() / 2.0)),
                                    self.world.fetch::<CreateMass>().0,
                                )
                            }
                            .create();
                            std::mem::drop(create_shape_opt);
                            self.world.insert(CreationData(None));
                        }
                        (ShapeInfo::Circle(_), true) => {
                            BodyBuilder {
                                translation: start_pos,
                                rotation: 0.0,
                                restitution: self.world.fetch::<CreateElasticity>().0,
                                friction: self.world.fetch::<CreateFriction>().0,
                                status,
                                ..BodyBuilder::from_world(
                                    &self.world,
                                    ShapeInfo::Circle(Some(mouse_drag_vec.norm().max(0.001))),
                                    self.world.fetch::<CreateMass>().0,
                                )
                            }
                            .create();
                            std::mem::drop(create_shape_opt);
                            self.world.insert(CreationData(None));
                        }
                        (ShapeInfo::Circle(_), false) => {
                            BodyBuilder {
                                translation: start_pos - mouse_drag_vec / 2.0,
                                rotation: 0.0,
                                restitution: self.world.fetch::<CreateElasticity>().0,
                                friction: self.world.fetch::<CreateFriction>().0,
                                status,
                                ..BodyBuilder::from_world(
                                    &self.world,
                                    ShapeInfo::Circle(Some(
                                        (mouse_drag_vec.norm() / 2.0).max(0.001),
                                    )),
                                    self.world.fetch::<CreateMass>().0,
                                )
                            }
                            .create();
                            std::mem::drop(create_shape_opt);
                            self.world.insert(CreationData(None));
                        }
                        (ShapeInfo::Polygon(_), _) => {}
                        (ShapeInfo::Polyline(Some(points)), _) => {
                            BodyBuilder {
                                restitution: self.world.fetch::<CreateElasticity>().0,
                                friction: self.world.fetch::<CreateFriction>().0,
                                status,
                                ..BodyBuilder::from_world(
                                    &self.world,
                                    ShapeInfo::Polyline(Some(points.clone())),
                                    self.world.fetch::<CreateMass>().0,
                                )
                            }
                            .create();
                            std::mem::drop(create_shape_opt);
                            self.world.insert(CreationData(None));
                        }
                        (ShapeInfo::Polyline(None), _) => unreachable!(),
                    }
                }
            }

            MouseButton::Right => {
                self.world.insert(ScalingGraph(false));
            }

            _ => {}
        }

        self.world.insert(resources::MouseStartPos(None));
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut ggez::Context,
        btn: KeyCode,
        keymods: KeyMods,
        _repeat: bool,
    ) {
        // hotkeys
        match (btn, keymods) {
            (KeyCode::B, KeyMods::NONE) => {
                self.world
                    .insert(CreationData(Some(ShapeInfo::Rectangle(None))));
            }
            (KeyCode::C, KeyMods::NONE) => {
                self.world
                    .insert(CreationData(Some(ShapeInfo::Circle(None))));
            }
            (KeyCode::Space, KeyMods::NONE) => {
                self.world.fetch_mut::<Paused>().toggle();
            }
            (KeyCode::S, KeyMods::NONE) => {
                self.world.fetch_mut::<CreateShapeStatic>().toggle();
            }
            (KeyCode::A, KeyMods::NONE) => {
                self.world.fetch_mut::<CreateShapeCentered>().toggle();
            }
            (KeyCode::D, KeyMods::SHIFT) => {
                self.delete_all();
            }
            (KeyCode::D, KeyMods::NONE) => {
                let found_sidepanel_entity =
                    self.imgui_wrapper.shown_menus.iter().find_map(|signal| {
                        if let UiChoice::SideMenu(entity) = signal {
                            Some(*entity)
                        } else {
                            None
                        }
                    });

                if let Some(sidepanel_entity) = found_sidepanel_entity {
                    self.delete_entity(sidepanel_entity);
                }
            }
            _ => {}
        }

        self.imgui_wrapper.update_key_down(btn, keymods);
    }

    fn key_up_event(&mut self, _ctx: &mut ggez::Context, btn: KeyCode, keymods: KeyMods) {
        self.imgui_wrapper.update_key_up(btn, keymods);
    }

    fn text_input_event(&mut self, _ctx: &mut ggez::Context, val: char) {
        self.imgui_wrapper.update_text(val);
    }
}
