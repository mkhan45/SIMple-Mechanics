use bevy_ecs::prelude::*;
use egui_macroquad::macroquad::prelude::*;
use rapier2d::prelude::{ShapeType, TypedShape};

use crate::physics::PhysicsRes;

pub fn draw_sys(physics_res: Res<PhysicsRes>) {
    let colliders = &physics_res.collider_set;
    let sf = physics_res.scale_factor;

    for (_handle, collider) in colliders.iter() {
        match collider.shape().as_typed_shape() {
            TypedShape::Ball(ball) => {
                let r = ball.radius / sf;
                let pos = collider.position().translation;

                draw_circle(pos.x / sf, pos.y / sf, r, WHITE);
            }
            _ => unimplemented!(),
        }
    }
}
