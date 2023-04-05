use ggez::glam::vec2;
use ggez::glam::Vec2;
use ggez::graphics::Color;
use ggez::graphics::DrawMode;
use ggez::graphics::Mesh;
use ggez::graphics::Rect;
use ggez::Context;
use ggez::GameError;
use rand::Rng;

use crate::Entity;

pub const BLOCK_WIDTH: f32 = 100.0;
pub const BLOCK_HEIGHT: f32 = 40.0;

pub struct Block {
    pub life: u8,
    pub pos: Vec2,
    pub shape: Mesh,
}

impl Block {
    pub fn new(ctx: &mut Context, position: Vec2) -> Result<Block, GameError> {
        let rect = Rect::new(0.0, 0.0, BLOCK_WIDTH, BLOCK_HEIGHT);

        let shape =
            Mesh::new_rectangle(ctx, DrawMode::fill(), rect, Color::from_rgb(210, 180, 140))?;

        Ok(Self {
            life: rand::thread_rng().gen_range(1..5),
            pos: position,
            shape,
        })
    }
}

impl Entity for Block {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        todo!()
    }

    fn draw(
        &self,
        ctx: &ggez::Context,
        canvas: &mut ggez::graphics::Canvas,
    ) -> Result<(), GameError> {
        canvas.draw(&self.shape, self.pos);

        Ok(())
    }

    fn dimensions(&self) -> (f32, f32) {
        (BLOCK_WIDTH, BLOCK_HEIGHT)
    }

    fn velocity(&self) -> ggez::glam::Vec2 {
        vec2(0.0, 0.0)
    }

    fn position(&self) -> ggez::glam::Vec2 {
        self.pos
    }
}
