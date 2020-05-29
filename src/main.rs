use ggez::event::EventHandler;

use specs::prelude::*;

use nalgebra as na;
use ncollide2d as nc;
use nphysics2d as np;

type Vector = nalgebra::Vector2<f32>;
type Point = nalgebra::Point2<f32>;

type MechanicalWorld = np::world::DefaultMechanicalWorld<f32>;
type BodySet = np::object::DefaultBodySet<f32>;
type GeometricalWorld = np::world::DefaultGeometricalWorld<f32>;
type ColliderSet = np::object::DefaultColliderSet<f32>;

type ShapeHandle = nc::shape::ShapeHandle<f32>;
type ColliderHandle = np::object::DefaultColliderHandle;
type RigidBody = np::object::RigidBody<f32>;
type RigidBodyDesc = np::object::RigidBodyDesc<f32>;

mod components;
use components::*;

struct MainState {
    world: specs::World,
}

impl MainState {
    fn add_body(&mut self, shape: ShapeHandle, body: RigidBody) {
        let body_handle = self
            .world
            .get_mut::<BodySet>()
            .expect("error getting DefaultBodySet")
            .insert(body);

        let coll = np::object::ColliderDesc::new(shape)
            .build(np::object::BodyPartHandle(body_handle, 0));

        let coll_handle = self
            .world
            .get_mut::<ColliderSet>()
            .expect("error getting DefaultColliderSet")
            .insert(coll);

        let specs_handle = self
            .world
            .create_entity()
            .with(PhysicsBody { body_handle })
            .with(Collider { coll_handle })
            .build();

        self.world
            .get_mut::<BodySet>()
            .expect("Error getting body set")
            .rigid_body_mut(body_handle)
            .unwrap()
            .set_user_data(Some(Box::new(specs_handle)));
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }
}

fn main() -> ggez::GameResult {
    // create a mutable reference to a `Context` and `EventsLoop`
    let (ctx, event_loop) = &mut ggez::ContextBuilder::new("Pong", "Fish").build().unwrap();

    let mut world = specs::World::new();

    let mechanical_world = MechanicalWorld::new(Vector::new(0.0, -9.81));
    let geometrical_world: GeometricalWorld = GeometricalWorld::new();
    let bodies: BodySet = BodySet::new();
    let colliders: ColliderSet = ColliderSet::new();

    world.insert(mechanical_world);
    world.insert(geometrical_world);
    world.insert(bodies);
    world.insert(colliders);

    // Make a mutable reference to `MainState`
    let main_state = &mut MainState { world };

    main_state.add_body(ShapeHandle::new(nc::shape::Ball::new(5.0)), RigidBodyDesc::new().build());

    // Start the game
    ggez::event::run(ctx, event_loop, main_state)
}
