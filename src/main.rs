use std::error::Error;

use snake_game::{app::App, config::Config};

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::default();
    let mut app = App::new(config)?;
    app.run()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
    use snake_game::{
        config::ConfigError,
        game::{state::GameState, world::GameWorld},
        input::{AppCommand, parse_key_event},
        models::{direction::Direction, position::Position},
    };

    fn test_config() -> Config {
        Config {
            width: 6,
            height: 6,
            initial_tick_ms: 100,
            minimum_tick_ms: 50,
            tick_reduction_ms: 10,
            foods_per_speedup: 2,
        }
    }

    #[test]
    fn test_config_validation() {
        let mut invalid = test_config();
        invalid.width = 3;
        assert_eq!(invalid.validate(), Err(ConfigError::WidthTooSmall));

        invalid = test_config();
        invalid.foods_per_speedup = 0;
        assert_eq!(invalid.validate(), Err(ConfigError::ZeroFoodsPerSpeedup));
    }

    #[test]
    fn test_direction_reversal_prevention() {
        let mut world = GameWorld::new(test_config());
        assert_eq!(world.snake().current_direction(), Direction::Right);

        // Sequence: Up then Down (Down should be rejected because next_direction becomes Up)
        world.change_direction(Direction::Up);
        assert_eq!(world.snake().next_direction(), Direction::Up);

        world.change_direction(Direction::Down);
        assert_eq!(world.snake().next_direction(), Direction::Up);
    }

    #[test]
    fn test_input_parsing() {
        let event = KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE);
        assert_eq!(parse_key_event(event), Some(AppCommand::Move(Direction::Up)));

        let release_event = KeyEvent {
            code: KeyCode::Char('w'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Release,
            state: crossterm::event::KeyEventState::NONE,
        };
        assert_eq!(parse_key_event(release_event), None);
    }

    #[test]
    fn test_position_movement() {
        let pos = Position::new(2, 2);
        assert_eq!(pos.moved(Direction::Up), Position::new(2, 1));
        assert_eq!(pos.moved(Direction::Down), Position::new(2, 3));
        assert_eq!(pos.moved(Direction::Left), Position::new(1, 2));
        assert_eq!(pos.moved(Direction::Right), Position::new(3, 2));
    }

    #[test]
    fn test_win_condition_when_board_full() {
        // 5x5 board -> playable cells = 3x3 = 9.
        let config = Config {
            width: 5,
            height: 5,
            initial_tick_ms: 100,
            minimum_tick_ms: 50,
            tick_reduction_ms: 10,
            foods_per_speedup: 1,
        };
        let mut world = GameWorld::new(config);

        // Advance until board fills up or win condition triggers
        for _ in 0..100 {
            if world.state() != GameState::Running {
                break;
            }
            if let Some(food) = world.food() {
                let fpos = food.position;
                let hpos = world.snake().head();
                // Teleport snake head directly to food to simulate instant eat
                if fpos.x > hpos.x {
                    world.change_direction(Direction::Right);
                } else if fpos.x < hpos.x {
                    world.change_direction(Direction::Left);
                } else if fpos.y > hpos.y {
                    world.change_direction(Direction::Down);
                } else {
                    world.change_direction(Direction::Up);
                }
            }
            world.update();
        }

        assert!(matches!(world.state(), GameState::Won | GameState::GameOver));
    }
}
