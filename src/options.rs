extern crate ggez;

use ggez::graphics::Color;

pub struct Options {
    pub grid_size: (i16, i16),
    pub grid_cell_size: (i16, i16),
    pub background_color: Color,
    pub snake_head_color: Color,
    pub snake_body_color: Color,
    pub food_color: Color,
    pub millis_per_update: u64,
}
