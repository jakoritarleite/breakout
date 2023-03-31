use ggez::{
    conf::Conf,
    event::{self, EventHandler},
    glam::Vec2,
    graphics::{self, Color, Rect},
    input::keyboard::KeyCode,
    Context, ContextBuilder, GameError,
};

fn main() -> Result<(), GameError> {
    let cfg = Conf::new();
    let (mut ctx, event_loop) = ContextBuilder::new("breakout", "Joao Koritar")
        .default_conf(cfg)
        .build()
        .expect("Could not create game context");

    let breakout = Breakout::new(&mut ctx).expect("Could not create game state");

    event::run(ctx, event_loop, breakout)
}

struct Player {
    pos_x: f32,
    shape: graphics::Mesh,
}

impl Player {
    pub fn new(ctx: &mut Context) -> Result<Player, GameError> {
        let rect = Rect::new(0.0, 0.0, 170.0, 30.0);
        let shape =
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, Color::BLACK)?;

        Ok(Self {
            // Align it in the middle of the screen
            pos_x: (ctx.gfx.size().0 / 2.0) - (rect.w / 2.0),
            shape,
        })
    }
}

struct Breakout {
    player: Player,
}

impl Breakout {
    pub fn new(ctx: &mut Context) -> Result<Breakout, GameError> {
        let player = Player::new(ctx)?;

        Ok(Self { player })
    }
}

impl EventHandler for Breakout {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        if ctx.keyboard.is_key_pressed(KeyCode::A) {
            self.player.pos_x -= 1.0;
        }

        if ctx.keyboard.is_key_pressed(KeyCode::D) {
            self.player.pos_x += 1.0;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from([0.1, 0.2, 0.3, 1.0]));

        canvas.draw(
            &self.player.shape,
            Vec2::new(self.player.pos_x, ctx.gfx.size().1 - 150.0),
        );

        canvas.finish(ctx)
    }
}
