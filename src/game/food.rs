extern crate ggez;
extern crate rand;

use crate::grid_position::GridPosition;
use crate::game::snake::Snake;
use crate::game::snake::SnakeTile;
use ggez::{graphics, Context, GameResult};
use rand::{thread_rng, Rng};

pub struct Food {
    pub pos: GridPosition,
    color: graphics::Color,
    grid_cell_size: (i16, i16),
}

impl Food {
    pub fn new(snake: &Snake, color: graphics::Color, grid_size: (i16, i16), grid_cell_size: (i16, i16)) -> Self {
        let random_pos;
        loop {
            let x = thread_rng().gen_range(0, grid_size.0);
            let y = thread_rng().gen_range(0, grid_size.1);
            if !snake.tiles.contains(&SnakeTile::new(x, y)) {
                random_pos = GridPosition { x, y };
                break;
            }
        }
        Food {
            pos: random_pos,
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
