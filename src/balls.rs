use ggez::glam::vec2;
use ggez::glam::Vec2;
use ggez::graphics::Color;
use ggez::graphics::DrawMode;
use ggez::graphics::Mesh;
use ggez::Context;
use ggez::GameError;
use rand::Rng;

use crate::Entity;

const BALL_SPEED: f32 = 300f32;
const BALL_RADIUS: f32 = 10f32;

pub struct Ball {
    pub vel: Vec2,
    pub pos_x: f32,
    pub pos_y: f32,
    pub shape: Mesh,
}

impl Ball {
    pub fn new(ctx: &mut Context) -> Result<Ball, GameError> {
        let shape = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            vec2(-BALL_RADIUS, -BALL_RADIUS),
            BALL_RADIUS,
            1.0,
            Color::WHITE,
        )?;

        let origin = (
            ctx.gfx.size().0 / 2.0 - BALL_RADIUS,
            ctx.gfx.size().1 - 150.0,
        );

        Ok(Self {
            vel: vec2(
                rand::thread_rng().gen_range(-1.0..1.0),
                rand::thread_rng().gen_range(-1.0..1.0),
            )
            .normalize(),
            pos_x: origin.0,
            pos_y: origin.1,
            shape,
        })
    }

    pub fn bounce(&mut self, ctx: &Context) {
        if self.pos_x < BALL_RADIUS {
            self.vel.x = 1.0;
        }

        if self.pos_x > ctx.gfx.size().0 - BALL_RADIUS {
            self.vel.x = -1.0;
        }

        if self.pos_y < BALL_RADIUS {
            self.vel.y = 1.0;
        }

        if self.pos_y > ctx.gfx.size().1 - BALL_RADIUS {
            self.vel.y = -1.0;
        }
    }
}

impl Entity for Ball {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let dt = ctx.time.delta().as_secs_f32();

        self.pos_x += self.vel.x * BALL_SPEED * dt;
        self.pos_y += self.vel.y * BALL_SPEED * dt;

        self.bounce(ctx);

        Ok(())
    }

    fn draw(
        &self,
        _ctx: &ggez::Context,
        canvas: &mut ggez::graphics::Canvas,
    ) -> Result<(), GameError> {
        canvas.draw(&self.shape, Vec2::new(self.pos_x, self.pos_y));

        Ok(())
    }
}
