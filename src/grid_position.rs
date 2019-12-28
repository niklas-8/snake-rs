extern crate ggez;

use ggez::{graphics, Context};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct GridPosition {
    pub x: i16,
    pub y: i16,
}

impl GridPosition {
    pub fn to_rect(
        &self,
        ctx: &mut Context,
        color: graphics::Color,
        grid_cell_size: (i16, i16),
    ) -> graphics::Mesh {
        graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                (self.x * grid_cell_size.0) as f32,
                (self.y * grid_cell_size.1) as f32,
                grid_cell_size.0 as f32,
                grid_cell_size.1 as f32,
            ),
            color,
        )
        .unwrap()
    }
}
