use ggez::event::EventHandler;
use ggez::glam::Vec2;
use ggez::graphics::{self, Color};
use ggez::input::keyboard::KeyCode;
use ggez::Context;
use ggez::GameError;

use crate::player::Player;

pub struct Breakout {
    pub player: Player,
}

impl Breakout {
    pub fn new(ctx: &mut Context) -> Result<Breakout, GameError> {
        let player = Player::new(ctx)?;

        Ok(Self { player })
    }
}

impl EventHandler for Breakout {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        if ctx.keyboard.is_key_pressed(KeyCode::A) {
            self.player.pos_x -= 1.0;
        }

        if ctx.keyboard.is_key_pressed(KeyCode::D) {
            self.player.pos_x += 1.0;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from([0.1, 0.2, 0.3, 1.0]));

        canvas.draw(
            &self.player.shape,
            Vec2::new(self.player.pos_x, ctx.gfx.size().1 - 150.0),
        );

        canvas.finish(ctx)
    }
}
