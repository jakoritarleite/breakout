use ggez::{graphics::Canvas, Context, GameError};

pub mod player;
pub mod state;

trait Entity {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError>;

    fn draw(&self, ctx: &Context, canvas: &mut Canvas) -> Result<(), GameError>;
}
