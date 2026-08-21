use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use std::io::{self, Stdout};

use crate::{game::world::GameWorld, models::position::Position};

pub fn draw_board(stdout: &mut Stdout, world: &GameWorld) -> io::Result<()> {
    let config = world.config();
    let width = config.width;
    let height = config.height;
    let food_pos = world.food().map(|f| f.position);

    for y in 0..height {
        queue!(stdout, MoveTo(0, y + 1))?;
        for x in 0..width {
            let pos = Position::new(x, y);

            if y == 0 || y == height - 1 || x == 0 || x == width - 1 {
                let ch = match (x, y) {
                    (0, 0) => '┌',
                    (px, 0) if px == width - 1 => '┐',
                    (0, py) if py == height - 1 => '└',
                    (px, py) if px == width - 1 && py == height - 1 => '┘',
                    (_, 0) => '─',
                    (_, py) if py == height - 1 => '─',
                    _ => '│',
                };
                queue!(stdout, SetForegroundColor(Color::DarkGrey), Print(ch), ResetColor)?;
            } else if Some(pos) == food_pos {
                queue!(stdout, SetForegroundColor(Color::Red), Print('●'), ResetColor)?;
            } else if let Some((idx, _)) =
                world.snake().body().iter().enumerate().find(|(_, p)| **p == pos)
            {
                if idx == 0 {
                    queue!(stdout, SetForegroundColor(Color::Green), Print('█'), ResetColor)?;
                } else {
                    queue!(stdout, SetForegroundColor(Color::DarkGreen), Print('█'), ResetColor)?;
                }
            } else {
                queue!(stdout, Print(' '))?;
            }
        }
    }
    Ok(())
}
