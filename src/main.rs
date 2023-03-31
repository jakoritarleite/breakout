use breakout::state::Breakout;
use ggez::conf::Conf;
use ggez::event;
use ggez::ContextBuilder;
use ggez::GameError;

fn main() -> Result<(), GameError> {
    let cfg = Conf::new();
    let (mut ctx, event_loop) = ContextBuilder::new("breakout", "Joao Koritar")
        .default_conf(cfg)
        .build()
        .expect("Could not create game context");

    let breakout = Breakout::new(&mut ctx).expect("Could not create game state");

    event::run(ctx, event_loop, breakout)
}
