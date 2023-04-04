use crate::balls::Ball;
use crate::Entity;
use ggez::glam::vec2;
use ggez::glam::Vec2;
use ggez::graphics::Canvas;
use ggez::graphics::Color;
use ggez::graphics::DrawMode;
use ggez::graphics::Mesh;
use ggez::graphics::Rect;
use ggez::input::keyboard::KeyCode;
use ggez::Context;
use ggez::GameError;

const PLAYER_WIDTH: f32 = 170f32;
const PLAYER_HEIGHT: f32 = 30f32;

pub struct Player {
    speed: Vec2,
    pub pos_x: f32,
    pub shape: Mesh,
    pub shots: Vec<Ball>,
}

impl Player {
    pub fn new(ctx: &mut Context) -> Result<Player, GameError> {
        let rect = Rect::new(
            -PLAYER_WIDTH * 0.5,
            -PLAYER_HEIGHT * 0.5,
            PLAYER_WIDTH,
            PLAYER_HEIGHT,
        );
        let shape = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, Color::WHITE)?;

        Ok(Self {
            speed: vec2(300.0, 0.0),
            // Align it in the middle of the screen
            pos_x: (ctx.gfx.size().0 * 0.5),
            shape,
            shots: vec![],
        })
    }
}

impl Entity for Player {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let dt = ctx.time.delta().as_secs_f32();

        if ctx.keyboard.is_key_pressed(KeyCode::A) {
            self.pos_x -= self.speed.x * dt;
        }

        if ctx.keyboard.is_key_pressed(KeyCode::D) {
            self.pos_x += self.speed.x * dt;
        }

        if ctx.keyboard.is_key_pressed(KeyCode::Space) {
            self.shots.push(Ball::new(ctx)?);
        }

        let player_width_half = PLAYER_WIDTH * 0.5;

        Self::clamp(
            &mut self.pos_x,
            player_width_half,
            ctx.gfx.size().0 - player_width_half,
        );

        // Loop over all balls
        for shot in self.shots.iter_mut() {
            shot.update(ctx)?;
        }

        Ok(())
    }

    fn draw(&self, ctx: &Context, canvas: &mut Canvas) -> Result<(), GameError> {
        canvas.draw(&self.shape, Vec2::new(self.pos_x, ctx.gfx.size().1 - 150.0));

        // Loop over all balls
        for shot in self.shots.iter() {
            shot.draw(ctx, canvas)?;
        }

        Ok(())
    }
}
