use specs::prelude::*;
use nphysics2d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};
use nalgebra as na;
use na::{Vector2, Point2, Isometry2};
use nphysics2d::object::{BodyStatus, RigidBodyDesc, ColliderDesc};
use nphysics2d::math::{Velocity, Inertia};
use ncollide2d::shape;

type RigidBody = nphysics2d::object::RigidBody<f32>;

pub struct PhysicsObject {
    rigidbody: RigidBody,
}
