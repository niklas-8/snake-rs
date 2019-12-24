pub mod grid_position;

pub const GRID_SIZE: (i16, i16) = (32, 20);
pub const GRID_CELL_SIZE: (i16, i16) = (20, 20);
const SCREEN_SIZE: (i16, i16) = (
    GRID_SIZE.0 * GRID_CELL_SIZE.0,
    GRID_SIZE.1 * GRID_CELL_SIZE.1,
);

pub fn size() -> (f32, f32) {
    (SCREEN_SIZE.0 as f32, SCREEN_SIZE.1 as f32)
}
