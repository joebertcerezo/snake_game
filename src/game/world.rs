use std::time::Duration;

use crate::{
    config::Config,
    game::state::GameState,
    models::{direction::Direction, food::Food, position::Position, snake::Snake},
};

pub struct GameWorld {
    config: Config,
    snake: Snake,
    food: Option<Food>,
    score: u32,
    state: GameState,
    current_tick_ms: u64,
}

impl GameWorld {
    pub fn new(config: Config) -> Self {
        let center = Position::new(config.width / 2, config.height / 2);
        let snake = Snake::new(center);
        let food = Food::spawn_random(&config, &snake);

        Self {
            config,
            snake,
            food,
            score: 0,
            state: GameState::Running,
            current_tick_ms: config.initial_tick_ms,
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn snake(&self) -> &Snake {
        &self.snake
    }

    pub fn food(&self) -> Option<&Food> {
        self.food.as_ref()
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn state(&self) -> GameState {
        self.state
    }

    pub fn current_tick_ms(&self) -> u64 {
        self.current_tick_ms
    }

    pub fn reset(&mut self) {
        let center = Position::new(self.config.width / 2, self.config.height / 2);
        self.snake = Snake::new(center);
        self.food = Food::spawn_random(&self.config, &self.snake);
        self.score = 0;
        self.state = GameState::Running;
        self.current_tick_ms = self.config.initial_tick_ms;
    }

    pub fn change_direction(&mut self, dir: Direction) {
        if self.state == GameState::Running {
            self.snake.change_direction(dir);
        }
    }

    pub fn toggle_pause(&mut self) {
        match self.state {
            GameState::Running => self.state = GameState::Paused,
            GameState::Paused => self.state = GameState::Running,
            _ => {}
        }
    }

    pub fn update(&mut self) {
        if self.state != GameState::Running {
            return;
        }

        let next_head = self.snake.next_head();

        // Boundary check
        if next_head.x == 0
            || next_head.x >= self.config.width - 1
            || next_head.y == 0
            || next_head.y >= self.config.height - 1
        {
            self.state = GameState::GameOver;
            return;
        }

        let is_eating = self.food.as_ref().map_or(false, |f| f.position == next_head);

        // Self-collision check
        if self.snake.self_collides_at(next_head, is_eating) {
            self.state = GameState::GameOver;
            return;
        }

        self.snake.advance(next_head, is_eating);

        if is_eating {
            self.score += 10;
            self.adjust_speed();
            self.food = Food::spawn_random(&self.config, &self.snake);

            if self.food.is_none() {
                self.state = GameState::Won;
            }
        }
    }

    fn adjust_speed(&mut self) {
        let speedups = (self.score / 10) / self.config.foods_per_speedup;
        let total_reduction = speedups as u64 * self.config.tick_reduction_ms;

        self.current_tick_ms = self
            .config
            .initial_tick_ms
            .saturating_sub(total_reduction)
            .max(self.config.minimum_tick_ms);
    }

    pub fn tick_rate(&self) -> Duration {
        Duration::from_millis(self.current_tick_ms)
    }
}
