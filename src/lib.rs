use ggez::glam::Vec2;
use ggez::graphics::Canvas;
use ggez::Context;
use ggez::GameError;

pub mod ball;
pub mod block;
pub mod ecs;
pub mod geometry;
pub mod physics;
pub mod player;
pub mod state;

pub trait Entity {
    fn velocity(&self) -> Vec2;

    fn position(&self) -> Vec2;

    fn dimensions(&self) -> (f32, f32);

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
