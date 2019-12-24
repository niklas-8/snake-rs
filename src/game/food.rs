extern crate ggez;
extern crate rand;

use crate::window::grid_position::GridPosition;
use crate::window::GRID_SIZE;
use ggez::{graphics, Context, GameResult};
use rand::{thread_rng, Rng};

const FOOD_COLOR: graphics::Color = graphics::Color::new(0.6, 0.8, 0.9, 1.0);

pub struct Food {
    pub pos: GridPosition,
}

impl Food {
    pub fn new() -> Self {
        Food {
            pos: GridPosition {
                x: thread_rng().gen_range(0, GRID_SIZE.0),
                y: thread_rng().gen_range(0, GRID_SIZE.1),
            },
        }
    }

    pub fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let rect = self.pos.to_rect(_ctx, FOOD_COLOR);
        graphics::draw(_ctx, &rect, graphics::DrawParam::default())?;
        Ok(())
    }
}
