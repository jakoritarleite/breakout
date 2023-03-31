use ggez::event::EventHandler;
use ggez::graphics::{self, Canvas, Color};

use ggez::Context;
use ggez::GameError;

use crate::player::Player;
use crate::Entity;

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
        update_entity(&mut self.player, ctx)?;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from([0.1, 0.2, 0.3, 1.0]));

        draw_entity(&self.player, ctx, &mut canvas)?;

        canvas.finish(ctx)
    }
}

fn update_entity<T: Entity>(entity: &mut T, ctx: &mut Context) -> Result<(), GameError> {
    entity.update(ctx)
}

fn draw_entity<T: Entity>(entity: &T, ctx: &Context, canvas: &mut Canvas) -> Result<(), GameError> {
    entity.draw(ctx, canvas)
}
