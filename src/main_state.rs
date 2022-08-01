use bevy_ecs::prelude::*;
use egui_macroquad::macroquad::prelude::*;
use rapier2d::{na::Vector2, prelude::{RigidBodyType, TypedShape}, parry::shape::Cuboid};

use crate::{
    physics::{physics_step_sys, PhysicsRes, Shape},
    draw::draw_sys,
};

pub struct MainState {
    pub world: World,
    physics_schedule: Schedule,
    draw_schedule: Schedule,
}

impl Default for MainState {
    fn default() -> Self {
        let world = {
            let mut world = World::new();
            world.insert_resource(PhysicsRes::default());

            crate::physics::BodyBuilder {
                position: Vector2::new(screen_width() / 2.0, 1.0),
                restitution: 0.7,
                ..Default::default()
            }
            .add_to_world(&mut world);

            crate::physics::BodyBuilder {
                position: Vector2::new(0.0, screen_height() - 10.0),
                typ: RigidBodyType::Fixed,
                shape: Shape::Rectangle(screen_width(), 10.0),
                restitution: 0.7,
                ..Default::default()
            }
            .add_to_world(&mut world);

            world
        };

        let physics_schedule = {
            let mut main_physics_schedule = Schedule::default();

            main_physics_schedule
                .add_stage("physics", SystemStage::single(physics_step_sys.system()));

            main_physics_schedule
        };

        let draw_schedule = {
            let mut draw_schedule = Schedule::default();

            draw_schedule
                .add_stage("draw", SystemStage::single(draw_sys.system()));

            draw_schedule
        };

        MainState {
            world,
            physics_schedule,
            draw_schedule,
        }
    }
}

impl MainState {
    pub fn update(&mut self) {
        self.physics_schedule.run(&mut self.world);
    }

    pub fn draw(&mut self) {
        clear_background(BLACK);
        self.draw_schedule.run(&mut self.world);
    }
}
