use std::{io::Result, time::Duration};

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use crate::models::direction::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppCommand {
    Move(Direction),
    Pause,
    Restart,
    Quit,
}

pub fn parse_key_event(key: KeyEvent) -> Option<AppCommand> {
    if key.kind != KeyEventKind::Press {
        return None;
    }

    match key.code {
        KeyCode::Esc => Some(AppCommand::Quit),
        KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
            Some(AppCommand::Move(Direction::Up))
        }
        KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => {
            Some(AppCommand::Move(Direction::Down))
        }
        KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('A') => {
            Some(AppCommand::Move(Direction::Left))
        }
        KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('D') => {
            Some(AppCommand::Move(Direction::Right))
        }
        KeyCode::Char('p') | KeyCode::Char('P') => Some(AppCommand::Pause),
        KeyCode::Char('r') | KeyCode::Char('R') => Some(AppCommand::Restart),
        _ => None,
    }
}

pub fn poll_command(timeout: Duration) -> Result<Option<AppCommand>> {
    if crossterm::event::poll(timeout)? {
        if let crossterm::event::Event::Key(key_event) = crossterm::event::read()? {
            return Ok(parse_key_event(key_event));
        }
    }
    Ok(None)
}
