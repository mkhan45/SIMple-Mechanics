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
    self, CreateElasticity, CreateFriction, CreateMass, CreateShapeStatic, MousePos, ShapeInfo,
};

use crate::gui::imgui_wrapper::{ImGuiWrapper, UiChoice, UiSignal};

use graphics::DrawMode;
use ncollide2d as nc;
use nphysics2d as np;
use resources::{CreateShapeCentered, CreationData, HiDPIFactor, MouseStartPos};

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
    pub fn process_gui_signals(&mut self) {
        self.imgui_wrapper
            .sent_signals
            .clone()
            .iter()
            .for_each(|signal| match signal {
                UiSignal::AddShape(shape_info) => {
                    self.world.insert(CreationData(Some(shape_info.clone())))
                }
                UiSignal::DeleteShape(entity) => {
                    self.delete_entity(*entity);
                    self.imgui_wrapper.remove_sidemenu(entity);
                }
                UiSignal::DeleteAll => {
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
            });
        self.imgui_wrapper.sent_signals.clear();
    }

    pub fn draw_creation_gui(&self, mesh_builder: &mut ggez::graphics::MeshBuilder) {
        let create_shape_opt = self.world.fetch::<CreationData>();
        let create_shape_data = create_shape_opt.0.as_ref();
        let create_shape_centered = self.world.fetch::<CreateShapeCentered>().0;

        if let (Some(create_shape_data), Some(start_pos)) =
            (&create_shape_data, self.world.fetch::<MouseStartPos>().0)
        {
            let mouse_pos = self.world.fetch::<MousePos>().0;
            let mouse_drag_vec = mouse_pos - start_pos;
            match (create_shape_data, create_shape_centered) {
                (ShapeInfo::Rectangle(_), true) => {
                    let v = mouse_drag_vec.abs();
                    mesh_builder.rectangle(
                        graphics::DrawMode::stroke(0.1),
                        graphics::Rect::new(
                            start_pos.x - v.x,
                            start_pos.y - v.y,
                            v.x * 2.0,
                            v.y * 2.0,
                        ),
                        graphics::WHITE,
                    );
                }
                (ShapeInfo::Rectangle(_), false) => {
                    let (start_pos, extents) = if mouse_drag_vec.y > 0.0 {
                        (start_pos, mouse_drag_vec)
                    } else {
                        (start_pos + mouse_drag_vec, -mouse_drag_vec)
                    };

                    mesh_builder.rectangle(
                        graphics::DrawMode::stroke(0.1),
                        graphics::Rect::new(start_pos.x, start_pos.y, extents.x, extents.y),
                        graphics::WHITE,
                    );
                }
                (ShapeInfo::Circle(_), true) => {
                    let r = mouse_drag_vec.magnitude();
                    mesh_builder.circle(
                        DrawMode::stroke(0.1),
                        [start_pos.x, start_pos.y],
                        r,
                        0.01,
                        graphics::WHITE,
                    );
                }
                (ShapeInfo::Circle(_), false) => {
                    let r = mouse_drag_vec.magnitude() / 2.0;
                    mesh_builder.circle(
                        DrawMode::stroke(0.1),
                        [
                            start_pos.x + mouse_drag_vec.x / 2.0,
                            start_pos.y + mouse_drag_vec.y / 2.0,
                        ],
                        r,
                        0.01,
                        graphics::WHITE,
                    );
                }
                _ => {}
            }
        }

        if let Some(ShapeInfo::Polygon(Some(points))) = &create_shape_data {
            let _ = mesh_builder.line(
                points
                    .iter()
                    .map(|p| [p.x, p.y])
                    .collect::<Vec<[f32; 2]>>()
                    .as_slice(),
                0.1,
                graphics::WHITE,
            );
        }
    }

    pub fn delete_entity(&mut self, entity: Entity) {
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

        self.world.delete_entity(entity).unwrap();
    }
}

impl<'a, 'b> EventHandler for MainState<'a, 'b> {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        {
            self.world.insert(resources::DT(ggez::timer::delta(ctx)));
            self.world.insert(resources::FPS(ggez::timer::fps(ctx)));
            self.world.maintain();
        }

        {
            self.process_gui_signals();
            self.lua_update();
        }

        {
            self.dispatcher.dispatch(&self.world);
        }

        {
            let info_displayed = self.world.read_storage::<InfoDisplayed>();
            let entities = self.world.entities();
            if let Some((_, entity)) = (&info_displayed, &entities).join().next() {
                self.imgui_wrapper
                    .shown_menus
                    .insert(UiChoice::SideMenu(Some(entity)));
            }
        }

        let geometrical_world = &mut self.world.fetch_mut::<GeometricalWorld>();
        let body_set = &mut *self.world.fetch_mut::<BodySet>();
        let collider_set = &mut *self.world.fetch_mut::<ColliderSet>();
        let joint_constraint_set = &mut *self.world.fetch_mut::<JointConstraintSet>();
        let force_generator_set = &mut *self.world.fetch_mut::<ForceGeneratorSet>();

        self.world.fetch_mut::<MechanicalWorld>().step(
            geometrical_world,
            body_set,
            collider_set,
            joint_constraint_set,
            force_generator_set,
        );

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        use graphics::Color;

        graphics::clear(ctx, graphics::BLACK);

        let mut mesh_builder = graphics::MeshBuilder::new();

        {
            let entities = self.world.entities();

            let colliders = self.world.read_storage::<Collider>();
            let colors = self.world.read_storage::<crate::components::Color>();
            let collider_set = self.world.fetch::<ColliderSet>();

            let selected = self.world.read_storage::<Selected>();

            (&colliders, &colors, &entities)
                .join()
                .for_each(|(collider_comp, color, e)| {
                    let collider = collider_set
                        .get(collider_comp.coll_handle)
                        .expect("error getting body to draw");

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

                        if selected.get(e).is_some() {
                            draw_circle(
                                &mut mesh_builder,
                                pos,
                                rot,
                                shape.radius(),
                                Color::new(1.0, 0.0, 0.0, 1.0),
                                true,
                            );
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

                        if selected.get(e).is_some() {
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
                });

            self.draw_creation_gui(&mut mesh_builder);
            if let Ok(mesh) = mesh_builder.build(ctx) {
                let _ = graphics::draw(ctx, &mesh, graphics::DrawParam::new());
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
    }

    fn mouse_motion_event(&mut self, ctx: &mut ggez::Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.imgui_wrapper.update_mouse_pos(x, y);

        let screen_size = graphics::drawable_size(ctx);
        let screen_coords = graphics::screen_coordinates(ctx);
        let mouse_point = Vector::new(
            x / screen_size.0 * screen_coords.w,
            y / screen_size.1 * screen_coords.h,
        );

        self.world.insert(resources::MousePos(mouse_point));
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut ggez::Context,
        btn: MouseButton,
        x: f32,
        y: f32,
    ) {
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
                let mut selected = self.world.write_storage::<Selected>();
                if let Some(entity) = get_hovered_shape(&self.world) {
                    selected.insert(entity, Selected::default()).unwrap();
                }

                {
                    let mut create_shape_data = self.world.fetch_mut::<CreationData>();
                    if let Some(ShapeInfo::Polygon(Some(points))) = create_shape_data.0.as_mut() {
                        points.push(mouse_point.into());
                    }
                }
            }
            MouseButton::Right => {
                let mut info_displayed = self.world.write_storage::<InfoDisplayed>();
                match get_hovered_shape(&self.world) {
                    Some(entity) => {
                        if info_displayed.get(entity).is_some() {
                            info_displayed.remove(entity).unwrap();
                            self.imgui_wrapper.remove_sidemenu(&entity);
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

                let create_shape_data = self.world.fetch::<CreationData>();
                if let Some(ShapeInfo::Polygon(Some(_points))) = &create_shape_data.0 {
                    // BodyBuilder {
                    // }.create();
                }
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
        if let MouseButton::Left = btn {
            {
                let mut selected = self.world.write_storage::<Selected>();
                selected.clear();
            }

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
                                ShapeInfo::Circle(Some((mouse_drag_vec.norm() / 2.0).max(0.001))),
                                self.world.fetch::<CreateMass>().0,
                            )
                        }
                        .create();
                        std::mem::drop(create_shape_opt);
                        self.world.insert(CreationData(None));
                    }
                    (ShapeInfo::Polygon(_), _) => {}
                }
            }
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
        match btn {
            KeyCode::B => {
                self.world
                    .insert(CreationData(Some(ShapeInfo::Rectangle(None))));
            }
            KeyCode::C => {
                self.world
                    .insert(CreationData(Some(ShapeInfo::Circle(None))));
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
