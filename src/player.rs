use crate::ball::Ball;
use crate::geometry;
use crate::physics::collision;
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
            shape,
            shots: vec![],
        })
    }
}

impl Entity for Player {
    fn velocity(&self) -> Vec2 {
        self.speed
    }

    fn position(&self) -> Vec2 {
        vec2(self.pos_x, self.pos_y)
    }

    fn dimensions(&self) -> (f32, f32) {
        (PLAYER_WIDTH, PLAYER_HEIGHT)
    }

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

        let player_pos = self.position();

        for shot in self.shots.iter_mut() {
            shot.update(ctx)?;

            let shot_pos = shot.position();
            let shot_dim = shot.dimensions();
            let shot_vel = shot.velocity();

            if let Some((_, _, w, h)) = geometry::intersection(
                shot_pos,
                shot_dim.0,
                shot_dim.0,
                player_pos,
                PLAYER_WIDTH_HALF,
                PLAYER_HEIGHT_HALF,
            ) {
                let (shot_pos, shot_vel) = collision::aabb(
                    shot_pos,
                    shot_dim.0,
                    shot_dim.1,
                    shot_vel,
                    player_pos,
                    PLAYER_WIDTH,
                    PLAYER_HEIGHT,
                    w,
                    h,
                );

                shot.pos_x = shot_pos.x;
                shot.pos_y = shot_pos.y;

                shot.vel.x = shot_vel.x;
                shot.vel.y = shot_vel.y;
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
