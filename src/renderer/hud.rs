use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use std::io::{self, Stdout};

use crate::game::{state::GameState, world::GameWorld};

pub fn draw_hud(stdout: &mut Stdout, world: &GameWorld) -> io::Result<()> {
    queue!(
        stdout,
        MoveTo(0, 0),
        SetForegroundColor(Color::Yellow),
        Print(format!(" Score: {}  |  Speed: {}ms", world.score(), world.current_tick_ms())),
        ResetColor
    )?;

    let bottom_y = world.config().height + 2;
    queue!(stdout, MoveTo(0, bottom_y))?;

    match world.state() {
        GameState::Running => {
            queue!(stdout, Print(" Arrow Keys/WASD: Move  |  P: Pause  |  ESC: Quit"))?;
        }
        GameState::Paused => {
            queue!(
                stdout,
                SetForegroundColor(Color::Yellow),
                Print(" *** PAUSED ***  (Press P to resume, ESC to quit)"),
                ResetColor
            )?;
        }
        GameState::GameOver => {
            queue!(
                stdout,
                SetForegroundColor(Color::Red),
                Print(" *** GAME OVER ***  (Press R to restart, ESC to quit)"),
                ResetColor
            )?;
        }
        GameState::Won => {
            queue!(
                stdout,
                SetForegroundColor(Color::Green),
                Print(" *** YOU WIN! ***  (Press R to restart, ESC to quit)"),
                ResetColor
            )?;
        }
    }

    Ok(())
}
