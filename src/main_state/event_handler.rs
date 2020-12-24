use super::body_builder::BodyBuilder;
use super::util::*;
use super::*;

use nphysics2d as np;

use crate::gui::draw_creation_gui_sys::DrawCreationGUISys;

use draw_shape_sys::DrawShapesSys;

use crate::resources::{
    self, Camera, CreateElasticity, CreateFriction, CreateMass, CreateShapeCentered,
    CreateShapeStatic, CreationData, GraphPosData, HiDPIFactor, MousePos, MouseStartPos,
    MovingGraph, Paused, ScalingGraph, ShapeInfo,
};

use ggez::{
    event::EventHandler,
    graphics,
    input::{
        keyboard::{KeyCode, KeyMods},
        mouse::MouseButton,
    },
};

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

        self.move_camera(ctx);
        self.update_sidemenu();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, graphics::BLACK);

        // the mesh builder batches all of the objects into a single mesh,
        // reducing draw calls.
        // It's not used for graphs or the GUI
        let mut mesh_builder = graphics::MeshBuilder::new();

        {
            let mut draw_shapes_sys = DrawShapesSys {
                mesh_builder: &mut mesh_builder,
            };
            draw_shapes_sys.run_now(&self.world);

            // draws the outlined circle and rect if you're creating an object
            // self.draw_creation_gui(&mut mesh_builder);
            let mut draw_creation_gui_sys = DrawCreationGUISys {
                mesh_builder: &mut mesh_builder,
            };
            draw_creation_gui_sys.run_now(&self.world);

            if let Ok(mesh) = mesh_builder.build(ctx) {
                // this _will_ error if there's an empty screen, so just ignore it.
                // Even if the error is something else it's pretty easy to tell
                // when something goes wrong here because no shapes will be drawn

                let camera = self.world.fetch::<Camera>();
                let drawparam = camera.make_drawparam();
                let _ = graphics::draw(ctx, &mesh, drawparam);
            }
        }

        // ideally this block should be refactored to a system for each of the graphs
        // and a system to combine them all
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
        use screen_resize_sys::ScreenResizeSys;

        let mut screen_resize_sys = ScreenResizeSys {
            height,
            width,
            ctx,
        };

        screen_resize_sys.run_now(&self.world);
    }

    fn mouse_motion_event(&mut self, ctx: &mut ggez::Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        // input mouse position data to specs world and imgui
        self.imgui_wrapper.update_mouse_pos(x, y);

        let prev_mouse_point = self.world.fetch::<MousePos>().0;

        let screen_size = graphics::drawable_size(ctx);
        let screen_coords = graphics::screen_coordinates(ctx);
        let camera = self.world.fetch::<Camera>();
        let mouse_point = Vector::new(
            ((x / screen_size.0 * screen_coords.w) - camera.pos.x) / camera.scale,
            ((y / screen_size.1 * screen_coords.h) - camera.pos.y) / camera.scale,
        );

        std::mem::drop(camera);
        self.world.insert(resources::MousePos(mouse_point));

        use ggez::input::mouse;
        let delta_mouse = mouse_point - prev_mouse_point;
        if mouse::button_pressed(ctx, mouse::MouseButton::Middle) {
            self.world.fetch_mut::<Camera>().translate(-delta_mouse);
        }
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
                            self.imgui_wrapper.remove_sidemenu();
                        }
                    }
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
                    let (translation, shape_info) = match (data, create_shape_centered) {
                        (ShapeInfo::Rectangle(_), true) => {
                            (start_pos, ShapeInfo::Rectangle(Some(mouse_drag_vec.abs())))
                        }
                        (ShapeInfo::Rectangle(_), false) => {
                            let mut start_pos = start_pos;
                            start_pos.x +=
                                mouse_drag_vec.x.abs() * (mouse_drag_vec.x < 0.0) as usize as f32;
                            start_pos.y +=
                                mouse_drag_vec.y.abs() * (mouse_drag_vec.y < 0.0) as usize as f32;
                            (
                                start_pos - mouse_drag_vec.abs() / 2.0,
                                ShapeInfo::Rectangle(Some(mouse_drag_vec.abs() / 2.0)),
                            )
                        }
                        (ShapeInfo::Circle(_), true) => (
                            start_pos,
                            ShapeInfo::Circle(Some(mouse_drag_vec.norm().max(0.001))),
                        ),
                        (ShapeInfo::Circle(_), false) => (
                            start_pos - mouse_drag_vec / 2.0,
                            ShapeInfo::Circle(Some((mouse_drag_vec.norm() / 2.0).max(0.001))),
                        ),
                        _ => todo!(),
                    };

                    BodyBuilder {
                        translation,
                        rotation: 0.0,
                        restitution: self.world.fetch::<CreateElasticity>().0,
                        friction: self.world.fetch::<CreateFriction>().0,
                        status,
                        ..BodyBuilder::from_world(
                            &self.world,
                            shape_info,
                            self.world.fetch::<CreateMass>().0,
                        )
                    }
                    .create();
                    std::mem::drop(create_shape_opt);
                    self.world.insert(CreationData(None));
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
                let found_sidepanel_entity = self.imgui_wrapper.find_sidemenu_entity();

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

    fn mouse_wheel_event(&mut self, _ctx: &mut ggez::Context, _x: f32, y: f32) {
        let focus = self.world.fetch::<MousePos>().0;
        self.world
            .fetch_mut::<Camera>()
            .change_scale(y * 0.05, focus);
    }
}
