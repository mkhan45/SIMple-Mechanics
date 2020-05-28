use ggez::{
    event::EventHandler,
};

mod components;

struct MainState {
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }
}

fn main() -> ggez::GameResult {
    // create a mutable reference to a `Context` and `EventsLoop`
    let (ctx, event_loop) = &mut ggez::ContextBuilder::new("Pong", "Fish").build().unwrap();

    // Make a mutable reference to `MainState`
    let main_state = &mut MainState {};

    // Start the game
    ggez::event::run(ctx, event_loop, main_state)
}

