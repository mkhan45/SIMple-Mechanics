use specs::prelude::*;

mod main_state;
use main_state::MainState;

mod resources;

use nalgebra as na;
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

mod systems;
use systems::SelectedMoveSys;

const SCREEN_X: f32 = 20.0;
const SCREEN_Y: f32 = 20.0;

fn main() -> ggez::GameResult {
    // create a mutable reference to a `Context` and `EventsLoop`
    let (ctx, event_loop) = &mut ggez::ContextBuilder::new("Pong", "Fish").build().unwrap();
    ggez::graphics::set_mode(ctx, ggez::conf::WindowMode::default().resizable(true))?;

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

    world.insert(resources::MousePos::default());

    world.register::<PhysicsBody>();
    world.register::<Collider>();
    world.register::<Selected>();

    let mut dispatcher = DispatcherBuilder::new()
        .with(SelectedMoveSys, "selected_move_sys", &[])
        .build();

    dispatcher.setup(&mut world);

    // Make a mutable reference to `MainState`
    let main_state = &mut MainState { world, dispatcher };

    let circle = RigidBodyDesc::new()
        .translation(Vector::new(15.25, 1.0))
        .mass(1.0)
        .enable_gravity(true)
        .build();
    main_state.add_body(
        ShapeHandle::new(nc::shape::Ball::new(2.0)),
        circle,
        0.5,
        0.5,
    );

    let rect = RigidBodyDesc::new()
        .mass(5.0)
        .translation(Vector::new(8.0, 0.0))
        .rotation(std::f32::consts::PI / 3.0)
        .build();
    main_state.add_body(
        ShapeHandle::new(nc::shape::Cuboid::new(Vector::new(2.0, 1.0))),
        rect,
        0.2,
        0.5,
    );

    let floor = RigidBodyDesc::new()
        .translation(Vector::new(0.0, SCREEN_Y))
        .status(np::object::BodyStatus::Static)
        .enable_gravity(false)
        .build();
    main_state.add_body(
        ShapeHandle::new(nc::shape::Cuboid::new(Vector::new(SCREEN_X * 5.0, 0.25))),
        floor,
        0.5,
        0.5,
    );

    let static_block = RigidBodyDesc::new()
        .translation(Vector::new(SCREEN_X / 3.0, SCREEN_Y / 2.0))
        .status(np::object::BodyStatus::Static)
        .enable_gravity(false)
        .build();
    main_state.add_body(
        ShapeHandle::new(nc::shape::Cuboid::new(Vector::new(0.1, 0.1))),
        static_block,
        0.5,
        0.5,
    );

    // Start the game
    ggez::event::run(ctx, event_loop, main_state)
}
