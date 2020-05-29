use specs::prelude::*;
use specs::Component;

use na::{Isometry2, Point2, Vector2};
use nalgebra as na;
use ncollide2d::shape;
use nphysics2d::math::{Inertia, Velocity};
use nphysics2d::object::{BodyStatus, ColliderDesc, RigidBodyDesc};
use nphysics2d::world::{DefaultGeometricalWorld, DefaultMechanicalWorld};

type RigidBody = nphysics2d::object::RigidBody<f32>;

#[derive(Component)]
#[storage(VecStorage)]
pub struct PhysicsObject {
    rigidbody: RigidBody,
}
