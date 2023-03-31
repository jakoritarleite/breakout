use ggez::graphics::Color;
use ggez::graphics::DrawMode;
use ggez::graphics::Mesh;
use ggez::graphics::Rect;
use ggez::Context;
use ggez::GameError;

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
