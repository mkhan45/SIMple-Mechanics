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
type JointConstraintSet = np::joint::DefaultJointConstraintSet<f32>;
type ForceGeneratorSet = np::force_generator::DefaultForceGeneratorSet<f32>;

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
        let body_handle = self.world.fetch_mut::<BodySet>().insert(body);

        let coll =
            np::object::ColliderDesc::new(shape).build(np::object::BodyPartHandle(body_handle, 0));

        let coll_handle = self.world.fetch_mut::<ColliderSet>().insert(coll);

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
        use ggez::graphics;

        graphics::clear(ctx, graphics::Color::new(0.0, 0.0, 0.0, 1.0));

        let mut mesh_builder = graphics::MeshBuilder::new();

        let colliders = self.world.read_storage::<Collider>();
        let collider_set = self.world.fetch::<ColliderSet>();

        colliders.join().for_each(|collider_comp| {
            let collider = collider_set
                .get(collider_comp.coll_handle)
                .expect("error getting body to draw");
            let pos = {
                let na_vector = collider.position().translation.vector;
                [na_vector.x, na_vector.y]
            };

            if collider.shape().is_shape::<nc::shape::Ball<f32>>() {
                let shape = collider
                    .shape()
                    .downcast_ref::<nc::shape::Ball<f32>>()
                    .expect("bad shape");
                mesh_builder.circle(
                    graphics::DrawMode::fill(),
                    pos,
                    shape.radius(),
                    0.05,
                    graphics::Color::new(1.0, 1.0, 1.0, 1.0),
                );
            }
        });

        let mesh = mesh_builder.build(ctx)?;
        graphics::draw(ctx, &mesh, graphics::DrawParam::new())?;

        graphics::present(ctx)?;

        Ok(())
    }
}

fn main() -> ggez::GameResult {
    // create a mutable reference to a `Context` and `EventsLoop`
    let (ctx, event_loop) = &mut ggez::ContextBuilder::new("Pong", "Fish").build().unwrap();

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

    world.register::<PhysicsBody>();
    world.register::<Collider>();

    // Make a mutable reference to `MainState`
    let main_state = &mut MainState { world };

    let circle = RigidBodyDesc::new()
        .translation(Vector::new(500.0, 1.0))
        .mass(1.0)
        .enable_gravity(true)
        .build();

    main_state.add_body(ShapeHandle::new(nc::shape::Ball::new(50.0)), circle);

    // Start the game
    ggez::event::run(ctx, event_loop, main_state)
}
