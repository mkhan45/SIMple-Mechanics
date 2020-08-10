use specs::prelude::*;

mod main_state;
use main_state::MainState;

mod resources;

mod lua;

mod gui;
use gui::graphs::SpeedGraph;
use gui::imgui_wrapper::ImGuiWrapper;

use ncollide2d as nc;
use nphysics2d as np;

type Vector = nalgebra::Vector2<f32>;
type Point = nalgebra::Point2<f32>;

type MechanicalWorld = np::world::DefaultMechanicalWorld<f32>;
type BodySet = np::object::DefaultBodySet<f32>;
type GeometricalWorld = np::world::DefaultGeometricalWorld<f32>;
type ColliderSet = np::object::DefaultColliderSet<f32>;
type JointConstraintSet = np::joint::DefaultJointConstraintSet<f32>;
type ForceGeneratorSet = np::force_generator::DefaultForceGeneratorSet<f32>;

type ShapeHandle = nc::shape::ShapeHandle<f32>;
type ColliderHandle = np::object::DefaultColliderHandle;
type RigidBody = np::object::RigidBody<f32>;
type RigidBodyDesc = np::object::RigidBodyDesc<f32>;

mod components;
use components::*;

use crate::{
    gui::graphs::{RotGraph, RotVelGraph, XPosGraph, XVelGraph, YPosGraph, YVelGraph},
    gui::gui_systems::*,
};
use resources::HiDPIFactor;

const SCREEN_X: f32 = 20.0;
const SCREEN_Y: f32 = 20.0;

fn main() -> ggez::GameResult {
    // create a mutable reference to a `Context` and `EventsLoop`
    let (ctx, event_loop) = &mut ggez::ContextBuilder::new("Physics", "Mikail Khan")
        .window_setup(ggez::conf::WindowSetup::default().title("Physics"))
        .build()
        .unwrap();
    ggez::graphics::set_mode(
        ctx,
        ggez::conf::WindowMode::default()
            .maximized(true)
            .resizable(true)
            .fullscreen_type(ggez::conf::FullscreenType::Windowed),
    )?;

    let mut world = specs::World::new();

    let mechanical_world = MechanicalWorld::new(Vector::new(0.0, 9.81));
    let geometrical_world: GeometricalWorld = GeometricalWorld::new();
    let bodies: BodySet = BodySet::new();
    let colliders: ColliderSet = ColliderSet::new();
    let joint_constraints = JointConstraintSet::new();
    let force_gens = ForceGeneratorSet::new();

    world.insert(mechanical_world);
    world.insert(geometrical_world);
    world.insert(bodies);
    world.insert(colliders);
    world.insert(joint_constraints);
    world.insert(force_gens);

    world.insert(resources::SaveSceneFilename("lua/scene.lua".to_string()));
    world.insert(resources::SaveGraphFilename("graphs.csv".to_string()));

    world.insert(resources::MousePos::default());
    world.insert(resources::MouseStartPos(None));

    world.insert(resources::CreationData(None));
    world.insert(resources::CreateMass(5.0));
    world.insert(resources::CreateFriction(0.5));
    world.insert(resources::CreateElasticity(0.2));
    world.insert(resources::CreateShapeCentered(true));
    world.insert(resources::CreateShapeStatic(false));

    world.insert(resources::FrameSteps(1));
    world.insert(resources::Paused(false));

    world.insert(resources::GraphPosData::default());
    world.insert(resources::MovingGraph(false));
    world.insert(resources::ScalingGraph(false));

    world.insert(resources::GraphMinMax(
        std::f32::NEG_INFINITY,
        std::f32::INFINITY,
    ));

    {
        let dimensions = event_loop.get_primary_monitor().get_dimensions();
        world.insert(resources::Resolution(Vector::new(
            dimensions.width as f32,
            dimensions.height as f32,
        )));
    }

    let lua = rlua::Lua::new();
    lua.context(|lua_ctx| {
        let globals = lua_ctx.globals();
        let shapes: Vec<rlua::Table> = Vec::new();
        globals.set("shapes", shapes).unwrap();
        globals.set("PAUSED", false).unwrap();
        globals.set("GRAVITY", 9.81).unwrap();
        globals.set("PI", std::f32::consts::PI).unwrap();
        globals.set("SCREEN_X", crate::SCREEN_X).unwrap();
        globals.set("SCREEN_Y", crate::SCREEN_Y).unwrap();

        lua_ctx
            .load(
                r#"
                    function add_shape(shape)
                        shapes[#shapes+1] = shape
                    end

                    function add_shapes(...)
                        for _, shape in ipairs{...} do
                            add_shape(shape)
                        end
                    end

                    function update()
                    end
                "#,
            )
            .exec()
            .unwrap();
    });
    world.insert(std::sync::Arc::new(std::sync::Mutex::new(lua)));
    world.insert(resources::FPS(60.0));
    world.insert(resources::DT(std::time::Duration::from_millis(16)));
    world.insert(resources::Selected(None));

    world.register::<PhysicsBody>();
    world.register::<Collider>();
    world.register::<InfoDisplayed>();
    world.register::<Color>();
    world.register::<Name>();
    world.register::<SpeedGraph>();
    world.register::<RotVelGraph>();
    world.register::<XVelGraph>();
    world.register::<YVelGraph>();
    world.register::<XPosGraph>();
    world.register::<YPosGraph>();
    world.register::<RotGraph>();

    let mut dispatcher = DispatcherBuilder::new()
        .with(SelectedMoveSys, "selected_move_sys", &[])
        .with(SpeedGraphSys::default(), "speed_graph_sys", &[])
        .with(RotVelGraphSys::default(), "rotvel_graph_sys", &[])
        .with(XPosGraphSys::default(), "x_pos_graph_sys", &[])
        .with(YPosGraphSys::default(), "y_pos_graph_sys", &[])
        .with(XVelGraphSys::default(), "x_vel_graph_sys", &[])
        .with(YVelGraphSys::default(), "y_vel_graph_sys", &[])
        .with(RotGraphSys::default(), "rot_graph_sys", &[])
        .with(MinMaxGraphSys, "graph_minmax_sys", &["speed_graph_sys"])
        .with(GraphTransformSys, "graph_transform_sys", &[])
        .build();

    dispatcher.setup(&mut world);

    // Make a mutable reference to `MainState`
    let hidpi_factor = event_loop.get_primary_monitor().get_hidpi_factor() as f32;
    let resolution = event_loop.get_primary_monitor().get_dimensions();
    world.insert(HiDPIFactor(hidpi_factor));
    let imgui_wrapper = ImGuiWrapper::new(
        ctx,
        hidpi_factor,
        Vector::new(resolution.width as f32, resolution.height as f32),
    );

    let main_state = &mut MainState {
        world,
        dispatcher,
        imgui_wrapper,
    };

    main_state.add_shapes_from_lua_file("lua/init.lua");
    main_state.lua_update();

    // Start the game
    ggez::event::run(ctx, event_loop, main_state)
}
