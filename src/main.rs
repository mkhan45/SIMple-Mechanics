use ggez::event::EventHandler;

use specs::prelude::*;

use nphysics2d::object::{DefaultBodySet, DefaultColliderSet};

use nphysics2d as np;
use ncollide2d as nc;
use nalgebra as na;

type Vector = nalgebra::Vector2<f32>;
type Point = nalgebra::Point2<f32>;

type MechanicalWorld = nphysics2d::world::DefaultMechanicalWorld<f32>;
type BodySet = nphysics2d::object::DefaultBodySet<f32>;
type GeometricalWorld = nphysics2d::world::DefaultGeometricalWorld<f32>;
type ColliderSet = nphysics2d::object::DefaultColliderSet<f32>;

type ShapeHandle = ncollide2d::shape::ShapeHandle<f32>;
type RigidBody = nphysics2d::object::RigidBody<f32>;
type RigidBodyDesc = nphysics2d::object::RigidBodyDesc<f32>;

mod components;

struct MainState {
    world: specs::World,
}

impl MainState {
    fn add_body(
        &mut self,
        shape: ShapeHandle,
        body: np::object::RigidBodyDesc<f32>,
    ) {
        let mut handle = self
            .world
            .get_mut::<DefaultBodySet<f32>>()
            .expect("error getting DefaultBodySet")
            .insert();

        let coll = nphysics2d::object::ColliderDesc::new(shape)
            .build(nphysics2d::object::BodyPartHandle(handle, 0));

        let specs_hand = self.world
            .get_mut::<DefaultColliderSet<f32>>()
            .expect("error getting DefaultColliderSet")
            .insert(coll);
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

    let mut mechanical_world = MechanicalWorld::new(Vector::new(0.0, -9.81));
    let mut geometrical_world: GeometricalWorld = GeometricalWorld::new();
    let mut bodies: BodySet = DefaultBodySet::new();
    let mut colliders: ColliderSet = DefaultColliderSet::new();

    world.insert(mechanical_world);
    world.insert(geometrical_world);
    world.insert(bodies);
    world.insert(colliders);

    // Make a mutable reference to `MainState`
    let main_state = &mut MainState { world };

    main_state.add_body(ShapeHandle::new(ncollide2d::shape::Ball(5.0)), np::object::RigidBodyDesc::new())

    // Start the game
    ggez::event::run(ctx, event_loop, main_state)
}
