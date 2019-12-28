extern crate ggez;

mod args;
mod game;
mod options;
mod grid_position;

use crate::game::GameState;
use ggez::{conf, event, ContextBuilder};

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_AUTHOR: &str = env!("CARGO_PKG_AUTHORS");

fn main() {
    let options;
    match args::get_options() {
        Ok(v) => options = v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    }
    let (ctx, event_loop) = &mut ContextBuilder::new(APP_NAME, APP_AUTHOR)
        .window_setup(conf::WindowSetup::default().title(APP_NAME))
        .window_mode(conf::WindowMode::default().dimensions(
            (options.grid_size.0 * options.grid_cell_size.0) as f32,
            (options.grid_size.1 * options.grid_cell_size.1) as f32,
        ))
        .build()
        .unwrap();
    let state = &mut GameState::new(options);
    event::run(ctx, event_loop, state).unwrap();
}
