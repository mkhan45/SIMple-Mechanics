use specs::prelude::*;
use specs::Component;

use nphysics2d::object::DefaultBodyHandle;

use crate::ColliderHandle;

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

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Selected;

#[derive(Debug, Copy, Clone, Component)]
#[storage(VecStorage)]
pub struct Color(pub ggez::graphics::Color);

#[derive(Debug, Clone, Component)]
#[storage(HashMapStorage)]
pub struct Name(pub String);
