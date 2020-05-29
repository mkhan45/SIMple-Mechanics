use specs::prelude::*;
use specs::Component;

use na::{Isometry2, Point2, Vector2};
use nalgebra as na;
use ncollide2d::shape;
use nphysics2d::math::{Inertia, Velocity};
use nphysics2d::object::{
    BodyStatus, ColliderDesc, DefaultBodyHandle, DefaultColliderSet, RigidBodyDesc,
};
use nphysics2d::world::{DefaultGeometricalWorld, DefaultMechanicalWorld};

use crate::{ColliderHandle, ColliderSet};

#[derive(Debug, Copy, Clone, Component)]
#[storage(VecStorage)]
pub struct PhysicsBody {
    pub body_handle: DefaultBodyHandle,
}

#[derive(Debug, Copy, Clone, Component)]
#[storage(VecStorage)]
pub struct Collider {
    pub coll_handle: ColliderHandle,
}
