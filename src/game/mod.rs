mod food;
mod snake;

use food::Food;
use ggez::{event, graphics, timer, Context, GameResult};
use snake::direction::Direction;
use snake::Snake;
use std::time::Duration;

const BACKGROUND_COLOR: graphics::Color = graphics::BLACK;
const UPDATES_PER_SECOND: f32 = 8.0;

pub struct GameState {
    snake: Snake,
    food: Food,
    game_over: bool,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            snake: Snake::new(1, 1, 2),
            food: Food::new(),
            game_over: false,
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let eaten_food_before = self.snake.eaten_food;
        self.snake.update(&self.food);
        if self.snake.eaten_food > eaten_food_before {
            self.food = Food::new();
        }
        self.game_over = self.snake.ate_itself;
        if self.game_over {
            println!("Game Over!");
            println!("Score: {}", self.snake.eaten_food);
            event::quit(_ctx);
        }
        timer::sleep(Duration::from_millis(
            (1.0 / UPDATES_PER_SECOND * 1000.0) as u64,
        ));
        timer::yield_now();
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        graphics::clear(_ctx, BACKGROUND_COLOR);
        self.snake.draw(_ctx)?;
        self.food.draw(_ctx)?;
        graphics::present(_ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: event::KeyCode,
        _keymods: event::KeyMods,
        _repeat: bool,
    ) {
        if let Some(dir) = Direction::from_keycode(keycode) {
            if dir.inverse() != self.snake.last_update_dir {
                self.snake.dir = dir;
            }
        } else if keycode == event::KeyCode::Q {
            event::quit(_ctx);
        }
    }
}
