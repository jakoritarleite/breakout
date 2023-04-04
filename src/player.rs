use crate::balls::Ball;
use crate::balls::BALL_RADIUS;
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
const PLAYER_WIDTH_HALF: f32 = PLAYER_WIDTH * 0.5;
const PLAYER_HEIGHT_HALF: f32 = PLAYER_HEIGHT * 0.5;

pub struct Player {
    speed: Vec2,
    pub pos_x: f32,
    pos_y: f32,
    rect: Rect,
    pub shape: Mesh,
    pub shots: Vec<Ball>,
}

impl Player {
    pub fn new(ctx: &mut Context) -> Result<Player, GameError> {
        let rect = Rect::new(
            -PLAYER_WIDTH_HALF,
            -PLAYER_HEIGHT_HALF,
            PLAYER_WIDTH,
            PLAYER_HEIGHT,
        );

        let shape = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, Color::WHITE)?;

        Ok(Self {
            speed: vec2(300.0, 0.0),
            // Align it in the middle of the screen
            pos_x: ctx.gfx.size().0 * 0.5,
            pos_y: ctx.gfx.size().1 - 150.0,
            rect,
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

        Self::clamp(
            &mut self.pos_x,
            PLAYER_WIDTH_HALF,
            ctx.gfx.size().0 - PLAYER_WIDTH_HALF,
        );

        self.shots.retain(|shot| shot.pos_y < ctx.gfx.size().1);

        for shot in self.shots.iter_mut() {
            shot.update(ctx)?;

            // Intersection system
            let left = (shot.pos_x - BALL_RADIUS).max(self.pos_x - PLAYER_WIDTH_HALF);
            let top = (shot.pos_y - BALL_RADIUS).max(self.pos_y - PLAYER_HEIGHT_HALF);

            let right = (shot.pos_x + BALL_RADIUS).min(self.pos_x + PLAYER_WIDTH_HALF);
            let bottom = (shot.pos_y + BALL_RADIUS).min(self.pos_y + PLAYER_HEIGHT_HALF);

            // (x, y, w, h)
            let intersection = (left, top, right - left, bottom - top);

            if right < left || bottom < top {
                continue;
            }

            // AABB Collision system
            let shot_center = vec2(shot.pos_x - BALL_RADIUS, shot.pos_y - BALL_RADIUS);
            let player_center = vec2(self.pos_x - PLAYER_WIDTH, self.pos_y - PLAYER_HEIGHT);

            let to = shot_center - player_center;
            let to_signum = to.signum();

            match intersection.2 > intersection.3 {
                true => {
                    shot.pos_y -= to_signum.y * intersection.3;
                    shot.vel.y = -to_signum.y * shot.vel.y.abs();
                }
                false => {
                    shot.pos_x -= to_signum.x * intersection.2;
                    shot.vel.x = -to_signum.x * shot.vel.x.abs();
                }
            }
        }

        Ok(())
    }

    fn draw(&self, ctx: &Context, canvas: &mut Canvas) -> Result<(), GameError> {
        canvas.draw(&self.shape, Vec2::new(self.pos_x, self.pos_y));

        // Loop over all balls
        for shot in self.shots.iter() {
            shot.draw(ctx, canvas)?;
        }

        Ok(())
    }
}
