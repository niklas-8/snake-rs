use ggez::event::KeyCode;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn from_keycode(key: KeyCode) -> Option<Self> {
        match key {
            KeyCode::Up => Some(Direction::Up),
            KeyCode::K => Some(Direction::Up),
            KeyCode::Down => Some(Direction::Down),
            KeyCode::J => Some(Direction::Down),
            KeyCode::Left => Some(Direction::Left),
            KeyCode::H => Some(Direction::Left),
            KeyCode::Right => Some(Direction::Right),
            KeyCode::L => Some(Direction::Right),
            _ => None,
        }
    }

    pub fn inverse(&self) -> Self {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
