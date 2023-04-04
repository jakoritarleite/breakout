use ggez::{graphics::Canvas, Context, GameError};

pub mod balls;
pub mod blocks;
pub mod player;
pub mod state;

trait Entity {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError>;

    fn draw(&self, ctx: &Context, canvas: &mut Canvas) -> Result<(), GameError>;

    fn clamp(coord: &mut f32, low: f32, high: f32) {
        if *coord < low {
            *coord = low;
        } else if *coord > high {
            *coord = high;
        }
    }
}
