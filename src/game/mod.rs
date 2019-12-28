mod food;
mod snake;

use crate::options::Options;
use food::Food;
use ggez::{event, graphics, timer, Context, GameResult};
use snake::direction::Direction;
use snake::Snake;
use std::time::Duration;

pub struct GameState {
    snake: Snake,
    food: Food,
    game_over: bool,
    options: Options,
}

impl GameState {
    pub fn new(options: Options) -> Self {
        GameState {
            snake: Snake::new(
                1,
                1,
                2,
                options.snake_head_color,
                options.snake_body_color,
                options.grid_size,
                options.grid_cell_size,
            ),
            food: Food::new(
                options.food_color,
                options.grid_size,
                options.grid_cell_size,
            ),
            game_over: false,
            options,
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let eaten_food_before = self.snake.eaten_food;
        self.snake.update(&self.food);
        if self.snake.eaten_food > eaten_food_before {
            self.food = Food::new(
                self.options.food_color,
                self.options.grid_size,
                self.options.grid_cell_size,
            );
        }
        self.game_over = self.snake.ate_itself;
        if self.game_over {
            println!("Game Over!");
            println!("Score: {}", self.snake.eaten_food);
            event::quit(ctx);
        }
        timer::sleep(Duration::from_millis(self.options.millis_per_update));
        timer::yield_now();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, self.options.background_color);
        self.snake.draw(ctx)?;
        self.food.draw(ctx)?;
        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: event::KeyCode,
        _keymods: event::KeyMods,
        _repeat: bool,
    ) {
        if let Some(dir) = Direction::from_keycode(keycode) {
            if dir.inverse() != self.snake.last_update_dir {
                self.snake.dir = dir;
            }
        } else if keycode == event::KeyCode::Q {
            event::quit(ctx);
        }
    }
}
