use bevy_ecs::prelude::*;
use egui_macroquad::{
    egui::{CtxRef, Key},
    macroquad::prelude::*,
};
use rapier2d::na::Vector2;

use crate::physics::{self, PhysicsRes, Shape};

#[derive(PartialEq, Debug)]
pub enum CreationState {
    Unstarted,
    Initiated(Shape, Vec<Vec2>),
}

pub fn create_body_sys(
    mut creation_state: ResMut<CreationState>,
    commands: Commands,
    physics_state: ResMut<PhysicsRes>,
) {
    match &mut *creation_state {
        CreationState::Unstarted if is_key_pressed(KeyCode::C) => {
            *creation_state = CreationState::Initiated(Shape::Circle(0.0), Vec::new())
        }

        CreationState::Unstarted if is_key_pressed(KeyCode::R) => {
            *creation_state = CreationState::Initiated(Shape::Rectangle(0.0, 0.0), Vec::new())
        }

        CreationState::Unstarted => {}

        CreationState::Initiated(Shape::Circle(_), clicked_points) => match &clicked_points[..] {
            &[p1] if is_mouse_button_pressed(MouseButton::Left) => {
                let p2: Vec2 = mouse_position().into();
                let r = (p2 - p1).length();

                physics::BodyBuilder {
                    shape: Shape::Circle(r),
                    position: Vector2::new(p1.x, p1.y),
                    ..Default::default()
                }
                .add_to_world_with_commands(commands, physics_state);

                *creation_state = CreationState::Unstarted;
            }

            _ if is_mouse_button_pressed(MouseButton::Left) => {
                clicked_points.push(mouse_position().into());
            }

            _ => {}
        },

        _ => todo!(),
    }
}

pub fn draw_creation_sys(creation_state: Res<CreationState>) {
    match &*creation_state {
        CreationState::Unstarted => {}

        CreationState::Initiated(Shape::Circle(_), clicked_points) => match &clicked_points[..] {
            &[p1] => {
                let p2: Vec2 = mouse_position().into();
                let r = (p2 - p1).length();

                draw_circle_lines(p1.x, p1.y, r, 3.0, WHITE);
            }

            _ => {}
        },

        _ => todo!(),
    }
}
