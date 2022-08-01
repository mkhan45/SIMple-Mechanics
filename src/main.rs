use egui_macroquad::macroquad;
use macroquad::prelude::*;

pub mod main_state;
pub mod physics;
pub mod draw;

#[macroquad::main("Mechanics")]
async fn main() {
    next_frame().await;

    let mut main_state = main_state::MainState::default();

    loop {
        main_state.update();
        main_state.draw();
        next_frame().await
    }
}
