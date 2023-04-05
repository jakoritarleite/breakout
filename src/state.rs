use ggez::event::EventHandler;
use ggez::glam::vec2;
use ggez::graphics::{self, Canvas, Color};

use ggez::Context;
use ggez::GameError;

use crate::block::{Block, BLOCK_HEIGHT, BLOCK_WIDTH};
use crate::player::Player;
use crate::Entity;

pub struct Breakout {
    pub player: Player,
    pub blocks: Vec<Block>,
}

impl Breakout {
    pub fn new(ctx: &mut Context) -> Result<Breakout, GameError> {
        let player = Player::new(ctx)?;
        let mut blocks = Vec::new();

        let (rows, columns) = (6, 6);

        let padding = 8.0;
        let total_block_size = vec2(BLOCK_WIDTH, BLOCK_HEIGHT) + vec2(padding, padding);

        let board_start_pos = vec2(
            (ctx.gfx.size().0 - (total_block_size.x * rows as f32)) * 0.5,
            50.0,
        );

        for i in 0..rows * columns {
            let block_x = (i % rows) as f32 * total_block_size.x;
            let block_y = (i / rows) as f32 * total_block_size.y;

            blocks.push(Block::new(ctx, board_start_pos + vec2(block_x, block_y))?)
        }

        Ok(Self { player, blocks })
    }
}

impl EventHandler for Breakout {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        update_entity(&mut self.player, ctx)?;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from([0.1, 0.2, 0.3, 1.0]));

        draw_entity(&self.player, ctx, &mut canvas)?;

        for block in self.blocks.iter() {
            draw_entity(block, ctx, &mut canvas)?;
        }

        canvas.finish(ctx)
    }
}

fn update_entity<T: Entity>(entity: &mut T, ctx: &mut Context) -> Result<(), GameError> {
    entity.update(ctx)
}

fn draw_entity<T: Entity>(entity: &T, ctx: &Context, canvas: &mut Canvas) -> Result<(), GameError> {
    entity.draw(ctx, canvas)
}
