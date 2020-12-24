use ggez::graphics;
use ggez::Context;
use specs::prelude::*;

use crate::resources::{Resolution, ScaleFac};

use crate::{Vector, SCREEN_X, SCREEN_Y};

pub struct ScreenResizeSys<'c> {
    pub height: f32,
    pub width: f32,
    pub ctx: &'c mut Context,
}

impl<'a, 'c> System<'a> for ScreenResizeSys<'c> {
    type SystemData = (Write<'a, Resolution>, Write<'a, ScaleFac>);

    fn run(&mut self, (mut resolution, mut scale_fac): Self::SystemData) {
        // making width increase with respect to the height reveals more things
        // making the height increase with respect to the width scales everything down

        let aspect_ratio = self.height / self.width;
        let initial_ratio = 1.0;

        if initial_ratio > aspect_ratio {
            let new_width = SCREEN_X / aspect_ratio;
            ggez::graphics::set_screen_coordinates(
                self.ctx,
                graphics::Rect::new(0.0, 0.0, new_width, SCREEN_Y),
            )
            .expect("error resizing");
        } else {
            let new_height = SCREEN_Y * aspect_ratio;
            graphics::set_screen_coordinates(
                self.ctx,
                graphics::Rect::new(0.0, 0.0, SCREEN_X, new_height),
            )
            .expect("error resizing");
        }

        resolution.0 = Vector::new(self.width, self.height);

        let screen_coords = graphics::screen_coordinates(self.ctx);
        scale_fac.0 = Vector::new(screen_coords.w / SCREEN_X, screen_coords.h / SCREEN_Y);
    }
}
