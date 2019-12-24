extern crate ggez;
extern crate rand;

mod game;
mod window;

use crate::game::GameState;
use ggez::{conf, event, ContextBuilder};

fn main() {
    let window_size = window::size();
    let (ctx, event_loop) = &mut ContextBuilder::new("snake-rs", "niklas-8")
        .window_setup(conf::WindowSetup::default().title("snake-rs"))
        .window_mode(conf::WindowMode::default().dimensions(window_size.0, window_size.1))
        .build()
        .unwrap();
    let state = &mut GameState::new();
    event::run(ctx, event_loop, state).unwrap();
}
