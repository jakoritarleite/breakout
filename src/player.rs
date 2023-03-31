use ggez::glam::Vec2;
use ggez::graphics::Canvas;
use ggez::graphics::Color;
use ggez::graphics::DrawMode;
use ggez::graphics::Mesh;
use ggez::graphics::Rect;
use ggez::input::keyboard::KeyCode;
use ggez::Context;
use ggez::GameError;

use crate::Entity;

pub struct Player {
    pub pos_x: f32,
    pub shape: Mesh,
}

impl Player {
    pub fn new(ctx: &mut Context) -> Result<Player, GameError> {
        let rect = Rect::new(0.0, 0.0, 170.0, 30.0);
        let shape = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, Color::BLACK)?;

        Ok(Self {
            // Align it in the middle of the screen
            pos_x: (ctx.gfx.size().0 / 2.0) - (rect.w / 2.0),
            shape,
        })
    }
}

impl Entity for Player {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        if ctx.keyboard.is_key_pressed(KeyCode::A) {
            self.pos_x -= 1.0;
        }

        if ctx.keyboard.is_key_pressed(KeyCode::D) {
            self.pos_x += 1.0;
        }

        Ok(())
    }

    fn draw(&self, ctx: &Context, canvas: &mut Canvas) -> Result<(), GameError> {
        canvas.draw(&self.shape, Vec2::new(self.pos_x, ctx.gfx.size().1 - 150.0));

        Ok(())
    }
}
