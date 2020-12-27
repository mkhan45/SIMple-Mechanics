use specs::prelude::*;

use crate::{BodySet, Collider, ColliderSet, ShapeHandle, Vector};

use crate::components::*;

use crate::resources::ShapeInfo;
use crate::RigidBodyDesc;

use ncollide2d as nc;
use nphysics2d as np;

pub struct BodyBuilder<'a> {
    pub body_set: Write<'a, BodySet>,
    pub collider_set: Write<'a, ColliderSet>,
    pub lazy_update: Read<'a, LazyUpdate>,
    pub entities: Entities<'a>,
    pub shape: ShapeHandle,
    pub mass: f32,
    pub translation: Vector,
    pub rotation: f32,
    pub velocity: Vector,
    pub rotvel: f32,
    pub status: np::object::BodyStatus,
    pub restitution: f32,
    pub friction: f32,
    pub color: ggez::graphics::Color,
    pub name: Option<String>,
    pub update_fn: Option<String>,
    pub collisions_enabled: bool,
}

impl<'a> BodyBuilder<'a> {
    pub fn from_world(world: &'a World, shape_info: ShapeInfo, mass: f32) -> Self {
        BodyBuilder::new(
            world.fetch_mut::<BodySet>().into(),
            world.fetch_mut::<ColliderSet>().into(),
            world.fetch::<LazyUpdate>().into(),
            world.entities(),
            shape_info,
            mass,
        )
    }

    pub fn new(
        body_set: Write<'a, BodySet>,
        collider_set: Write<'a, ColliderSet>,
        lazy_update: Read<'a, LazyUpdate>,
        entities: Entities<'a>,
        shape_info: ShapeInfo,
        mass: f32,
    ) -> Self {
        let shape = match shape_info {
            ShapeInfo::Circle(Some(r)) => ShapeHandle::new(nc::shape::Ball::new(r)),
            ShapeInfo::Rectangle(Some(v)) => ShapeHandle::new(nc::shape::Cuboid::new(v)),
            ShapeInfo::Polygon(Some(points)) => {
                ShapeHandle::new(nc::shape::ConvexPolygon::try_new(points).unwrap())
            }
            ShapeInfo::Polyline(Some(points)) => {
                ShapeHandle::new(nc::shape::Polyline::new(points, None))
            }
            _ => panic!("Invalid shape info without data"),
        };

        BodyBuilder {
            body_set,
            collider_set,
            lazy_update,
            entities,
            shape,
            mass,
            translation: Vector::new(0.0, 0.0),
            rotation: 0.0,
            velocity: Vector::new(0.0, 0.0),
            rotvel: 0.0,
            status: np::object::BodyStatus::Dynamic,
            restitution: 0.2,
            friction: 0.5,
            color: ggez::graphics::WHITE,
            name: None,
            update_fn: None,
            collisions_enabled: true,
        }
    }

    pub fn create(mut self) -> Entity {
        let body = RigidBodyDesc::new()
            .mass(self.mass)
            .translation(self.translation)
            .rotation(self.rotation)
            .velocity(np::math::Velocity::new(self.velocity, self.rotvel))
            .status(self.status)
            .build();

        let body_handle = self.body_set.insert(body);

        let coll = np::object::ColliderDesc::new(self.shape)
            .density(1.0)
            .set_material(np::material::MaterialHandle::new(
                np::material::BasicMaterial::new(self.restitution, self.friction),
            ))
            .set_is_sensor(!self.collisions_enabled)
            .build(np::object::BodyPartHandle(body_handle, 0));

        let coll_handle = self.collider_set.insert(coll);

        let mut specs_handle = self
            .lazy_update
            .create_entity(&self.entities)
            .with(PhysicsBody { body_handle })
            .with(Collider { coll_handle })
            .with(Color(self.color));

        if let Some(n) = self.name {
            specs_handle = specs_handle.with(Name(n));
        }

        if let Some(f) = self.update_fn {
            specs_handle = specs_handle.with(UpdateFunction(f));
        }

        let specs_handle = specs_handle.entity;

        self.body_set
            .rigid_body_mut(body_handle)
            .unwrap()
            .set_user_data(Some(Box::new(specs_handle)));

        self.collider_set
            .get_mut(coll_handle)
            .unwrap()
            .set_user_data(Some(Box::new(specs_handle)));

        specs_handle
    }
}
