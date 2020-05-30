use smallvec::SmallVec;

use ggez::event::EventHandler;

use specs::prelude::*;

use crate::{
    BodySet, Collider, ColliderSet, ForceGeneratorSet, GeometricalWorld, JointConstraintSet,
    MechanicalWorld, Point, RigidBody, ShapeHandle,
};
use crate::{SCREEN_X, SCREEN_Y};

use crate::components::*;

use ncollide2d as nc;
use nphysics2d as np;

pub struct MainState {
    pub world: specs::World,
}

impl MainState {
    pub fn add_body(
        &mut self,
        shape: ShapeHandle,
        body: RigidBody,
        restitution: f32,
        friction: f32,
    ) {
        let body_handle = self.world.fetch_mut::<BodySet>().insert(body);

        let coll = np::object::ColliderDesc::new(shape)
            .density(1.0)
            .set_material(np::material::MaterialHandle::new(
                np::material::BasicMaterial::new(restitution, friction),
            ))
            .build(np::object::BodyPartHandle(body_handle, 0));

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
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
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
                mesh_builder.circle(
                    graphics::DrawMode::fill(),
                    pos,
                    shape.radius(),
                    0.01,
                    graphics::Color::new(1.0, 1.0, 1.0, 1.0),
                );

                mesh_builder.circle(
                    graphics::DrawMode::fill(),
                    [
                        pos[0] + shape.radius() * rot.cos() * 0.75,
                        pos[1] + shape.radius() * rot.sin() * 0.75,
                    ],
                    shape.radius() * 0.15,
                    0.01,
                    graphics::Color::new(0.0, 0.0, 0.0, 1.0),
                );
            } else if collider.shape().is_shape::<nc::shape::Cuboid<f32>>() {
                let shape = collider
                    .shape()
                    .downcast_ref::<nc::shape::Cuboid<f32>>()
                    .expect("bad shape");

                if rot == 0.0 {
                    mesh_builder.rectangle(
                        graphics::DrawMode::fill(),
                        graphics::Rect::new(
                            pos[0] - shape.half_extents().x,
                            pos[1] - shape.half_extents().y,
                            shape.half_extents().x * 2.0,
                            shape.half_extents().y * 2.0,
                        ),
                        graphics::Color::new(1.0, 1.0, 1.0, 1.0),
                    );
                } else {
                    let rot_cos = rot.cos();
                    let rot_sin = rot.sin();

                    let _points = [
                        Point::new(
                            pos[0] - shape.half_extents().x,
                            pos[1] - shape.half_extents().y,
                        ),
                        Point::new(
                            pos[0] + shape.half_extents().x,
                            pos[1] - shape.half_extents().y,
                        ),
                        Point::new(
                            pos[0] + shape.half_extents().x,
                            pos[1] + shape.half_extents().y,
                        ),
                        Point::new(
                            pos[0] - shape.half_extents().x,
                            pos[1] + shape.half_extents().y,
                        ),
                    ]
                    .iter()
                    .map(|point| {
                        [
                            rot_cos * (point.x - pos[0]) - rot_sin * (point.y - pos[1]) + pos[0],
                            rot_sin * (point.x - pos[0]) + rot_cos * (point.y - pos[1]) + pos[1],
                        ]
                    })
                    .collect::<SmallVec<[[f32; 2]; 4]>>();

                    let points = _points.as_slice();

                    mesh_builder
                        .polygon(
                            graphics::DrawMode::fill(),
                            points,
                            graphics::Color::new(1.0, 1.0, 1.0, 1.0),
                        )
                        .expect("error drawing rotated rect");
                };
            }
        });

        let mesh = mesh_builder.build(ctx)?;
        graphics::draw(ctx, &mesh, graphics::DrawParam::new())?;

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
    }
}
