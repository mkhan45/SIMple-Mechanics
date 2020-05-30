use smallvec::SmallVec;

use ggez::event::EventHandler;
use ggez::graphics;

use specs::prelude::*;

use crate::{
    BodySet, Collider, ColliderSet, ForceGeneratorSet, GeometricalWorld, JointConstraintSet,
    MechanicalWorld, Point, RigidBody, ShapeHandle, Vector,
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

                draw_circle(&mut mesh_builder, pos, rot, shape.radius());
            } else if collider.shape().is_shape::<nc::shape::Cuboid<f32>>() {
                let shape = collider
                    .shape()
                    .downcast_ref::<nc::shape::Cuboid<f32>>()
                    .expect("bad shape");

                draw_rect(&mut mesh_builder, pos, rot, *shape.half_extents());
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

fn draw_circle(mesh_builder: &mut ggez::graphics::MeshBuilder, pos: [f32; 2], rot: f32, rad: f32) {
    mesh_builder.circle(
        graphics::DrawMode::fill(),
        pos,
        rad,
        0.01,
        graphics::Color::new(1.0, 1.0, 1.0, 1.0),
    );

    mesh_builder.circle(
        graphics::DrawMode::fill(),
        [
            pos[0] + rad * rot.cos() * 0.75,
            pos[1] + rad * rot.sin() * 0.75,
        ],
        rad * 0.15,
        0.01,
        graphics::Color::new(0.0, 0.0, 0.0, 1.0),
    );
}

fn draw_rect(
    mesh_builder: &mut ggez::graphics::MeshBuilder,
    center_pos: [f32; 2],
    rot: f32,
    half_extents: Vector,
) {
    let rot_cos = rot.cos();
    let rot_sin = rot.sin();

    // rect points in clockwise (important for ggez)
    let _points = [
        Point::new(
            center_pos[0] - half_extents.x,
            center_pos[1] - half_extents.y,
        ),
        Point::new(
            center_pos[0] + half_extents.x,
            center_pos[1] - half_extents.y,
        ),
        Point::new(
            center_pos[0] + half_extents.x,
            center_pos[1] + half_extents.y,
        ),
        Point::new(
            center_pos[0] - half_extents.x,
            center_pos[1] + half_extents.y,
        ),
    ]
    .iter()
    .map(|point| {
        // new x position is cos(theta) * (p.x - c.x) - sin(theta) * (p.y - c.y) + c.x
        // new y position is sin(theta) * (p.x - c.x) + cos(theta) * (p.y - c.y) + c.y
        [
            rot_cos * (point.x - center_pos[0]) - rot_sin * (point.y - center_pos[1])
                + center_pos[0],
            rot_sin * (point.x - center_pos[0])
                + rot_cos * (point.y - center_pos[1])
                + center_pos[1],
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
}
