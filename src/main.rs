extern crate ggez;
extern crate rand;

mod args;
mod game;
mod options;
mod screen;

use crate::game::GameState;
use ggez::{conf, event, ContextBuilder};

fn main() {
    let options;
    match args::get_options() {
        Ok(v) => options = v,
        Err(e) => {
            println!("{}", e);
            return;
        },
    }
    let window_size = screen::size();
    let (ctx, event_loop) = &mut ContextBuilder::new("snake-rs", "niklas-8")
        .window_setup(conf::WindowSetup::default().title("snake-rs"))
        .window_mode(conf::WindowMode::default().dimensions(window_size.0, window_size.1))
        .build()
        .unwrap();
    let state = &mut GameState::new();
    event::run(ctx, event_loop, state).unwrap();
}
