use specs::prelude::*;

use nphysics2d::{world::DefaultMechanicalWorld, na::Vector2};

#[derive(Clone)]
pub struct MechanicalWorld(nphysics2d::world::DefaultMechanicalWorld);

impl Default for MechanicalWorld {
    fn default() -> Self {
        MechanicalWorld(DefaultMechanicalWorld::new(Vector2<f32>::new(0.0, 9.81)))
    }
}
