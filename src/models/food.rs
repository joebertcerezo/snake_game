use rand::RngExt;

use crate::{
    config::Config,
    models::{position::Position, snake::Snake},
};

pub struct Food {
    pub position: Position,
}

impl Food {
    pub fn spawn_random(config: &Config, snake: &Snake) -> Option<Self> {
        let mut available = Vec::new();

        let min_x = 1;
        let max_x = config.width.saturating_sub(2);
        let min_y = 1;
        let max_y = config.height.saturating_sub(2);

        if max_x < min_x || max_y < min_y {
            return None;
        }

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let pos = Position::new(x, y);
                if !snake.intersects_position(pos) {
                    available.push(pos);
                }
            }
        }

        if available.is_empty() {
            return None;
        }

        let index = rand::rng().random_range(0..available.len());
        Some(Self { position: available[index] })
    }
}
