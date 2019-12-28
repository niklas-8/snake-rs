extern crate ggez;
extern crate rand;

use crate::grid_position::GridPosition;
use ggez::{graphics, Context, GameResult};
use rand::{thread_rng, Rng};

pub struct Food {
    pub pos: GridPosition,
    color: graphics::Color,
    grid_cell_size: (i16, i16),
}

impl Food {
    pub fn new(color: graphics::Color, grid_size: (i16, i16), grid_cell_size: (i16, i16)) -> Self {
        Food {
            pos: GridPosition {
                x: thread_rng().gen_range(0, grid_size.0),
                y: thread_rng().gen_range(0, grid_size.1),
            },
            color,
            grid_cell_size,
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let rect = self.pos.to_rect(ctx, self.color, self.grid_cell_size);
        graphics::draw(ctx, &rect, graphics::DrawParam::default())?;
        Ok(())
    }
}
