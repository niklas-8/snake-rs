pub mod direction;

use crate::game::food::Food;
use crate::window::grid_position::GridPosition;
use crate::window::GRID_SIZE;
use direction::Direction;
use ggez::{graphics, Context, GameResult};
use std::collections::LinkedList;

const SNAKE_HEAD_COLOR: graphics::Color = graphics::WHITE;
const SNAKE_BODY_COLOR: graphics::Color = graphics::Color::new(0.8, 0.8, 0.8, 1.0);

pub struct Snake {
    tiles: LinkedList<SnakeTile>,
    pub dir: Direction,
    pub last_update_dir: Direction,
    pub ate_itself: bool,
    pub eaten_food: i16,
}

impl Snake {
    pub fn new(x: i16, y: i16, len: i16) -> Self {
        let mut list = LinkedList::new();
        for i in 0..len {
            list.push_back(SnakeTile::new(x - i, y));
        }
        Snake {
            tiles: list,
            dir: Direction::Right,
            last_update_dir: Direction::Right,
            ate_itself: false,
            eaten_food: 0,
        }
    }

    fn eats_itself(&self) -> bool {
        let mut is_first_tile = true;
        for tile in self.tiles.iter() {
            if !is_first_tile && tile == self.tiles.front().unwrap() {
                return true;
            }
            is_first_tile = false;
        }
        false
    }

    fn eats_food(&self, food: &Food) -> bool {
        return self.tiles.front().unwrap().pos == food.pos;
    }

    pub fn update(&mut self, food: &Food) {
        let new_tile = SnakeTile::new_from_move(self.tiles.front().unwrap().pos, self.dir);
        self.tiles.push_front(new_tile);
        self.ate_itself = self.eats_itself();
        if self.eats_food(food) {
            self.eaten_food += 1;
        } else {
            self.tiles.pop_back();
        }
        self.last_update_dir = self.dir;
    }

    pub fn draw(&self, _ctx: &mut Context) -> GameResult<()> {
        let mut color = SNAKE_HEAD_COLOR;
        for tile in self.tiles.iter() {
            let rect = tile.pos.to_rect(_ctx, color);
            color = SNAKE_BODY_COLOR;
            graphics::draw(_ctx, &rect, graphics::DrawParam::default())?;
        }
        Ok(())
    }
}

#[derive(PartialEq, Eq)]
struct SnakeTile {
    pos: GridPosition,
}

impl SnakeTile {
    fn new(x: i16, y: i16) -> Self {
        SnakeTile {
            pos: GridPosition { x, y },
        }
    }

    fn new_from_move(pos: GridPosition, dir: Direction) -> Self {
        match dir {
            Direction::Up => SnakeTile::new(pos.x, (pos.y - 1).modulo(GRID_SIZE.1)),
            Direction::Down => SnakeTile::new(pos.x, (pos.y + 1).modulo(GRID_SIZE.1)),
            Direction::Left => SnakeTile::new((pos.x - 1).modulo(GRID_SIZE.0), pos.y),
            Direction::Right => SnakeTile::new((pos.x + 1).modulo(GRID_SIZE.0), pos.y),
        }
    }
}

trait ModuloSigned {
    fn modulo(&self, n: Self) -> Self;
}

impl<T> ModuloSigned for T
where
    T: std::ops::Add<Output = T> + std::ops::Rem<Output = T> + Clone,
{
    fn modulo(&self, n: T) -> T {
        (self.clone() % n.clone() + n.clone()) % n.clone()
    }
}
