#![feature(move_ref_pattern)]
use specs::prelude::*;

mod main_state;
use main_state::MainState;

mod gui;
mod lua;
mod resources;

mod components;
use components::*;

mod types;
use types::*;

use lua::update_fn_sys::LuaUpdateFnSys;

use gui::{imgui_wrapper::ImGuiWrapper, systems::*};
use resources::HiDPIFactor;

use main_state::physics_sys::PhysicsSys;

const SCREEN_X: f32 = 20.0;
const SCREEN_Y: f32 = 20.0;

fn main() -> ggez::GameResult {
    // create a mutable reference to a ggez `Context` and `EventsLoop`
    let (ctx, event_loop) = &mut ggez::ContextBuilder::new("SIMple Physics", "Mikail Khan")
        .window_setup(ggez::conf::WindowSetup::default().title("SIMple Physics"))
        .build()
        .unwrap();

    // the specs world that almost all data goes in
    let mut world = specs::World::new();

    // the nphysics mechanical world stores actual physics stuff
    let mechanical_world = MechanicalWorld::new(Vector::new(0.0, 9.81));

    // the geometrical world from ncollide deals with collisions
    let geometrical_world: GeometricalWorld = GeometricalWorld::new();

    // the body set and collider set contain the rigid body data for the actual sim
    let bodies: BodySet = BodySet::new();
    let colliders: ColliderSet = ColliderSet::new();

    // right now the sim doesn't use any constraints or force generators
    // so these are just left as is
    let joint_constraints = JointConstraintSet::new();
    let force_gens = ForceGeneratorSet::new();

    // insert all the physics stuff into the specs world for use later
    world.insert(mechanical_world);
    world.insert(geometrical_world);
    world.insert(bodies);
    world.insert(colliders);
    world.insert(joint_constraints);
    world.insert(force_gens);

    // setting up defaults
    world.insert(resources::SaveSceneFilename("lua/scene.lua".to_string()));
    world.insert(resources::SaveGraphFilename("graphs.csv".to_string()));

    world.insert(resources::MousePos::default());
    world.insert(resources::MouseStartPos(None));
    world.insert(resources::ScaleFac::default());

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
        // init screen size to be the size of the smallest available monitor
        // Ideally this should be the monitor that it actually starts on
        // but probably 90% of users only have one monitor and it's easier
        // to drag a small window to big screen than a big window to small
        // screen
        let smallest_monitor = event_loop
            .get_available_monitors()
            .min_by_key(|monitor| monitor.get_dimensions().width as usize)
            .expect("error getting smallest monitor");
        let dimensions = smallest_monitor.get_dimensions();
        world.insert(resources::Resolution(Vector::new(
            dimensions.width as f32,
            dimensions.height as f32,
        )));
    }

    // new_lua_res() does a lot of stuff behind the scenes to
    // set up the Lua context
    world.insert(lua::new_lua_res());

    world.insert(resources::FPS(60.0));
    world.insert(resources::DT(std::time::Duration::from_millis(16)));
    world.insert(resources::Timestep(0.016));
    world.insert(resources::Selected(None));

    world.insert(resources::Camera::default());

    // many components aren't used in proper specs systems, so just
    // register them all manually.
    world.register::<Collider>();
    world.register::<InfoDisplayed>();

    // The specs dispatcher takes a bunch of systems and tries to
    // run them in parallel. dispatcher.dispatch() is run every frame
    let mut dispatcher = DispatcherBuilder::new()
        .with(PhysicsSys, "physics_sys", &[])
        .with(SelectedMoveSys, "selected_move_sys", &["physics_sys"])
        .with(
            SpeedGraphSys::default(),
            "speed_graph_sys",
            &["physics_sys"],
        )
        .with(
            RotVelGraphSys::default(),
            "rotvel_graph_sys",
            &["physics_sys"],
        )
        .with(XPosGraphSys::default(), "x_pos_graph_sys", &["physics_sys"])
        .with(YPosGraphSys::default(), "y_pos_graph_sys", &["physics_sys"])
        .with(XVelGraphSys::default(), "x_vel_graph_sys", &["physics_sys"])
        .with(YVelGraphSys::default(), "y_vel_graph_sys", &["physics_sys"])
        .with(RotGraphSys::default(), "rot_graph_sys", &["physics_sys"])
        .with(MinMaxGraphSys, "graph_minmax_sys", &["physics_sys"])
        .with(GraphTransformSys, "graph_transform_sys", &["physics_sys"])
        .with(LuaUpdateFnSys, "lua_update_fn", &["physics_sys"])
        .build();

    dispatcher.setup(&mut world);

    // More defaults relating to ggez and imgui
    let hidpi_factor = event_loop.get_primary_monitor().get_hidpi_factor() as f32;
    let resolution = event_loop.get_primary_monitor().get_dimensions();
    world.insert(HiDPIFactor(hidpi_factor));

    let imgui_wrapper = ImGuiWrapper::new(
        ctx,
        hidpi_factor,
        Vector::new(resolution.width as f32, resolution.height as f32),
    );

    let dimensions = world.fetch::<resources::Resolution>().0;
    ggez::graphics::set_mode(
        ctx,
        ggez::conf::WindowMode::default()
            .resizable(true)
            .min_dimensions(960.0, 720.0)
            .dimensions(dimensions.x, dimensions.y)
            .fullscreen_type(ggez::conf::FullscreenType::Windowed),
    )?;

    let screen_coords = ggez::graphics::screen_coordinates(ctx);
    world.insert(resources::ScaleFac(Vector::new(
        screen_coords.w / SCREEN_X,
        screen_coords.h / SCREEN_Y,
    )));

    // the MainState runs the main loop and input handling
    let main_state = &mut MainState {
        world,
        dispatcher,
        imgui_wrapper,
    };

    main_state.add_shapes_from_lua_file("lua/init.lua");
    main_state.lua_update();

    ggez::event::run(ctx, event_loop, main_state)
}
