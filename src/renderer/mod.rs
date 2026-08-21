pub mod board;
pub mod hud;

use crossterm::{
    cursor::MoveTo,
    queue,
    terminal::{Clear, ClearType},
};
use std::io::{self, Write, stdout};

use board::draw_board;
use hud::draw_hud;

use crate::game::world::GameWorld;

pub fn render_frame(world: &GameWorld) -> io::Result<()> {
    let mut stdout = stdout();

    queue!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;
    draw_board(&mut stdout, world)?;
    draw_hud(&mut stdout, world)?;

    stdout.flush()?;
    Ok(())
}
