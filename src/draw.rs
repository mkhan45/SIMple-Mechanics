use bevy_ecs::prelude::*;
use egui_macroquad::macroquad::prelude::*;
use rapier2d::prelude::{ShapeType, TypedShape};

use crate::physics::PhysicsRes;

pub fn draw_sys(physics_res: Res<PhysicsRes>) {
    let colliders = &physics_res.collider_set;
    let sf = physics_res.scale_factor;

    for (_handle, collider) in colliders.iter() {
        let pos = {
            let v = collider.position().translation;
            Vec2::new(v.x, v.y) / sf
        };

        match collider.shape().as_typed_shape() {
            TypedShape::Ball(ball) => {
                let r = ball.radius / sf;

                draw_circle(pos.x, pos.y, r, WHITE);
            }
            TypedShape::Cuboid(cuboid) => {
                let extents = cuboid.half_extents;
                let w = extents.x * 2.0 / sf;
                let h = extents.y * 2.0 / sf;

                draw_rectangle(pos.x - w / 2.0, pos.y - h / 2.0, w, h, WHITE);
            }
            _ => unimplemented!(),
        }
    }
}
