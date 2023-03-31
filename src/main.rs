use ggez::{
    event::{self, EventHandler},
    graphics::{self, Color},
    Context, ContextBuilder, GameError,
};

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("breakout", "Joao Koritar")
        .build()
        .expect("Could not create game context");

    let breakout = Breakout::new(&mut ctx);

    event::run(ctx, event_loop, breakout);
}

struct Breakout;

impl Breakout {
    pub fn new(ctx: &mut Context) -> Self {
        Self
    }
}

impl EventHandler for Breakout {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        canvas.finish(ctx)
    }
}
