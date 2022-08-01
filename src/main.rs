use egui_macroquad::macroquad;
use macroquad::prelude::*;

pub mod physics;
pub mod main_state;

#[macroquad::main("Mechanics")]
async fn main() {
    next_frame().await;

    let mut _main_state = main_state::MainState::default();

    loop {
        next_frame().await
    }
}
