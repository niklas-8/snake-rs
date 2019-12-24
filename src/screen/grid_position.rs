extern crate ggez;

use crate::screen::GRID_CELL_SIZE;
use ggez::{graphics, Context};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct GridPosition {
    pub x: i16,
    pub y: i16,
}

impl GridPosition {
    pub fn to_rect(&self, _ctx: &mut Context, color: graphics::Color) -> graphics::Mesh {
        graphics::Mesh::new_rectangle(
            _ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                (self.x * GRID_CELL_SIZE.0) as f32,
                (self.y * GRID_CELL_SIZE.1) as f32,
                GRID_CELL_SIZE.0 as f32,
                GRID_CELL_SIZE.1 as f32,
            ),
            color,
        )
        .unwrap()
    }
}
