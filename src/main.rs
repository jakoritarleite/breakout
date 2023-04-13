use std::borrow::BorrowMut;

use breakout::ecs::entity::Entity;
use breakout::ecs::world::World;
use breakout::geometry;
use breakout::physics::collision;
use breakout_macros::Component;
use ggez::conf::Conf;
use ggez::event;
use ggez::event::EventHandler;
use ggez::glam::vec2;
use ggez::glam::Vec2;
use ggez::graphics;
use ggez::graphics::Canvas;
use ggez::graphics::Color;
use ggez::graphics::DrawMode;
use ggez::graphics::Mesh;
use ggez::graphics::Rect;
use ggez::input::keyboard::KeyCode;
use ggez::Context;
use ggez::ContextBuilder;
use ggez::GameError;
use rand::Rng;

const PLAYER_WIDTH: f32 = 170f32;
const PLAYER_HEIGHT: f32 = 30f32;

const BLOCK_WIDTH: f32 = 100.0;
const BLOCK_HEIGHT: f32 = 40.0;
const BLOCK_LIFE: u8 = 5;

const BALL_SPEED: f32 = 300f32;
const BALL_RADIUS: f32 = 10f32;

fn main() -> Result<(), GameError> {
    let cfg = Conf::new();
    let (mut ctx, event_loop) = ContextBuilder::new("breakout", "Joao Koritar")
        .default_conf(cfg)
        .build()
        .expect("Could not create game context");

    let state = GameState::new(&mut ctx).expect("Could not create game state");

    event::run(ctx, event_loop, state)
}

struct GameState {
    world: World,
    player: Entity,
    blocks: Vec<Entity>,
    balls: Vec<Entity>,
}

#[derive(Debug, Clone, Component)]
struct Life(pub u8);

#[derive(Debug, Clone, Component)]
struct Position(pub Vec2);

#[derive(Debug, Clone, Component)]
struct Shape(pub Mesh);

#[derive(Debug, Clone, Component)]
struct Velocity(pub Vec2);

impl GameState {
    pub fn new(ctx: &mut Context) -> Result<Self, GameError> {
        let mut world = World::new();

        let player = spawn_player(&mut world, ctx)?;
        let blocks = spawn_blocks(&mut world, ctx)?;

        Ok(Self {
            world,
            player,
            blocks,
            balls: vec![],
        })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let dt = ctx.time.delta().as_secs_f32();

        check_player_collisions(self, ctx, dt)?;
        check_block_collisions(self, ctx, dt)?;
        update_player(self, ctx, dt)?;
        update_balls(self, ctx, dt)?;
        update_blocks(self, ctx, dt)?;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from([0.1, 0.2, 0.3, 1.0]));

        draw_entities(self, &mut canvas);

        canvas.finish(ctx)
    }
}

fn clamp(coord: &mut f32, low: f32, high: f32) {
    if *coord < low {
        *coord = low;
    } else if *coord > high {
        *coord = high;
    }
}

fn spawn_player(world: &mut World, ctx: &mut Context) -> Result<Entity, GameError> {
    let rect = Rect::new(0.0, 0.0, PLAYER_WIDTH, PLAYER_HEIGHT);

    Ok(world.spawn((
        Position(vec2(
            (ctx.gfx.size().0 * 0.5) - (PLAYER_WIDTH * 0.5),
            ctx.gfx.size().1 - 150.0,
        )),
        Shape(Mesh::new_rectangle(
            &ctx.gfx,
            DrawMode::fill(),
            rect,
            Color::WHITE,
        )?),
        Velocity(vec2(300.0, 0.0)),
    )))
}

fn spawn_blocks(world: &mut World, ctx: &mut Context) -> Result<Vec<Entity>, GameError> {
    let mut blocks = Vec::new();

    let (rows, columns) = (7, 7);

    let padding = 5.0;
    let total_block_size = vec2(BLOCK_WIDTH, BLOCK_HEIGHT) + vec2(padding, padding);

    let board_start_pos = vec2(
        (ctx.gfx.size().0 - (total_block_size.x * rows as f32)) * 0.5,
        50.0,
    );

    let rect = Rect::new(0.0, 0.0, BLOCK_WIDTH, BLOCK_HEIGHT);

    for i in 0..rows * columns {
        let block_x = (i % rows) as f32 * total_block_size.x;
        let block_y = (i / rows) as f32 * total_block_size.y;

        let entity = world.spawn((
            Life(BLOCK_LIFE),
            Position(board_start_pos + vec2(block_x, block_y)),
            Shape(Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                rect,
                Color::from_rgb(210, 180, 140),
            )?),
        ));

        blocks.push(entity);
    }

    Ok(blocks)
}

fn spawn_ball(world: &mut World, ctx: &mut Context) -> Result<Entity, GameError> {
    let circle = Mesh::new_circle(
        ctx,
        DrawMode::fill(),
        vec2(0.0, 0.0),
        BALL_RADIUS,
        0.1,
        Color::WHITE,
    )?;

    Ok(world.spawn((
        Position(vec2(
            ctx.gfx.size().0 / 2.0 - BALL_RADIUS,
            ctx.gfx.size().1 - 225.0,
        )),
        Velocity(
            vec2(
                rand::thread_rng().gen_range(-1.0..1.0),
                rand::thread_rng().gen_range(-1.0..1.0),
            )
            .normalize(),
        ),
        Shape(circle),
    )))
}

fn draw_entities(gs: &mut GameState, canvas: &mut Canvas) {
    let mut query = gs.world.query::<(&Position, &Shape)>();

    match query.get(&gs.world, gs.player.clone()) {
        (Some(position), Some(shape)) => canvas.draw(&shape.0, position.0),
        _ => panic!("Could not find components to draw Player"),
    }

    for block in gs.blocks.iter() {
        match query.get(&gs.world, block.clone()) {
            (Some(position), Some(shape)) => canvas.draw(&shape.0, position.0),
            _ => panic!("Could not find components to draw Block {:?}", block),
        }
    }

    for ball in gs.balls.iter() {
        match query.get(&gs.world, ball.clone()) {
            (Some(position), Some(shape)) => canvas.draw(&shape.0, position.0),
            _ => panic!("Could not find components to draw Ball {:?}", ball),
        }
    }
}

fn update_player(gs: &mut GameState, ctx: &mut Context, dt: f32) -> Result<(), GameError> {
    let mut query = gs.world.query::<(&mut Position, &Velocity)>();

    match query.get(&gs.world, gs.player.clone()) {
        (Some(mut position), Some(velocity)) => {
            let position = position.borrow_mut();

            if ctx.keyboard.is_key_pressed(KeyCode::A) {
                position.0.x -= velocity.0.x * dt;
            }

            if ctx.keyboard.is_key_pressed(KeyCode::D) {
                position.0.x += velocity.0.x * dt;
            }

            clamp(&mut position.0.x, 0.0, ctx.gfx.size().0 - PLAYER_WIDTH);
        }
        _ => panic!("Could not find components to update Player"),
    }

    if ctx.keyboard.is_key_pressed(KeyCode::Space) {
        if gs.balls.len() < 1 {
            let ball = spawn_ball(&mut gs.world, ctx)?;
            gs.balls.push(ball);
        }
    }

    Ok(())
}

fn update_blocks(gs: &mut GameState, _ctx: &mut Context, _dt: f32) -> Result<(), GameError> {
    let mut query = gs.world.query::<&Life>();

    let mut should_destroy = Vec::new();

    for block in gs.blocks.iter() {
        match query.get(&gs.world, block.clone()) {
            Some(life) => {
                if life.0 == 0 {
                    should_destroy.push(block.clone());
                    // TODO: also remove from world
                }
            }
            None => panic!("Could not find Block({:?}) life component", block),
        }
    }

    gs.blocks.retain(|block| !should_destroy.contains(block));

    Ok(())
}

fn update_balls(gs: &mut GameState, ctx: &mut Context, dt: f32) -> Result<(), GameError> {
    let mut query = gs.world.query::<(&mut Position, &mut Velocity)>();

    let mut should_destroy = Vec::new();

    for ball in gs.balls.iter() {
        match query.get(&gs.world, ball.clone()) {
            (Some(mut position), Some(mut velocity)) => {
                position.0.x += velocity.0.x * BALL_SPEED * dt;
                position.0.y += velocity.0.y * BALL_SPEED * dt;

                if position.0.x < BALL_RADIUS {
                    velocity.0.x = 1.0;
                }

                if position.0.x > ctx.gfx.size().0 - BALL_RADIUS {
                    velocity.0.x = -1.0;
                }

                if position.0.y < BALL_RADIUS {
                    velocity.0.y = 1.0;
                }

                if position.0.y < ctx.gfx.size().1 {
                    should_destroy.push(ball.clone());
                    // TODO: also remove from world
                }
            }
            _ => panic!("Could not find components to update Player"),
        }
    }

    gs.balls.retain(|ball| should_destroy.contains(ball));

    Ok(())
}

fn check_player_collisions(
    gs: &mut GameState,
    _ctx: &mut Context,
    _dt: f32,
) -> Result<(), GameError> {
    let mut query = gs.world.query::<(&mut Position, &mut Velocity)>();

    let player_position = match query.get(&gs.world, gs.player.clone()) {
        (Some(position), Some(_)) => position,
        _ => panic!("Could not find components to check Player collisions"),
    };

    for ball in gs.balls.iter() {
        match query.get(&gs.world, ball.clone()) {
            (Some(mut position), Some(mut velocity)) => {
                if let Some((_, _, w, h)) = geometry::intersection(
                    position.0,
                    BALL_RADIUS,
                    BALL_RADIUS,
                    player_position.0,
                    PLAYER_WIDTH,
                    PLAYER_HEIGHT,
                ) {
                    let (ball_position, ball_velocity) = collision::aabb(
                        position.0,
                        BALL_RADIUS,
                        BALL_RADIUS,
                        velocity.0,
                        player_position.0,
                        PLAYER_WIDTH,
                        PLAYER_HEIGHT,
                        w,
                        h,
                    );

                    position.0.x = ball_position.x;
                    position.0.y = ball_position.y;

                    velocity.0.x = ball_velocity.x;
                    velocity.0.y = ball_velocity.y;
                }
            }
            _ => panic!("Could not find components to update Ball {:?}", ball),
        }
    }

    Ok(())
}

fn check_block_collisions(
    gs: &mut GameState,
    _ctx: &mut Context,
    _dt: f32,
) -> Result<(), GameError> {
    let mut ball_query = gs.world.query::<(&mut Position, &mut Velocity)>();
    let mut block_query = gs.world.query::<(&Position, &mut Life)>();

    for ball in gs.balls.iter() {
        match ball_query.get(&gs.world, ball.clone()) {
            (Some(mut position), Some(mut velocity)) => {
                for block in gs.blocks.iter() {
                    match block_query.get(&gs.world, block.clone()) {
                        (Some(block_position), Some(mut life)) => {
                            if let Some((_, _, w, h)) = geometry::intersection(
                                position.0,
                                BALL_RADIUS,
                                BALL_RADIUS,
                                block_position.0,
                                BLOCK_WIDTH,
                                BLOCK_HEIGHT,
                            ) {
                                let (ball_position, ball_velocity) = collision::aabb(
                                    position.0,
                                    BALL_RADIUS,
                                    BALL_RADIUS,
                                    velocity.0,
                                    block_position.0,
                                    BLOCK_WIDTH,
                                    BLOCK_HEIGHT,
                                    w,
                                    h,
                                );

                                position.0.x = ball_position.x;
                                position.0.y = ball_position.y;

                                velocity.0.x = ball_velocity.x;
                                velocity.0.y = ball_velocity.y;

                                life.0 -= 1;
                            }
                        }
                        _ => panic!("Could not find block components"),
                    }
                }
            }
            _ => panic!("Could not find components to check ball collisions"),
        }
    }

    Ok(())
}
