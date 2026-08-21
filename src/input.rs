use crossterm::event::KeyEvent;

use crate::models::direction::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppCommand {
    Move(Direction),
    Pause,
    Restart,
    Quit,
}

pub fn parse_key_event(key: KeyEvent) -> Option<AppCommand> {
    match key.code {
        crossterm::event::KeyCode::Up => Some(AppCommand::Move(Direction::Up)),
        crossterm::event::KeyCode::Down => Some(AppCommand::Move(Direction::Down)),
        crossterm::event::KeyCode::Left => Some(AppCommand::Move(Direction::Left)),
        crossterm::event::KeyCode::Right => Some(AppCommand::Move(Direction::Right)),
        crossterm::event::KeyCode::Char('p') => Some(AppCommand::Pause),
        crossterm::event::KeyCode::Char('r') => Some(AppCommand::Restart),
        crossterm::event::KeyCode::Char('q') => Some(AppCommand::Quit),
        _ => None,
    }
}
