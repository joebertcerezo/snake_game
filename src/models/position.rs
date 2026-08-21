use crate::models::direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Position {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    pub fn moved(self, direction: direction::Direction) -> Self {
        match direction {
            direction::Direction::Up => Self { x: self.x, y: self.y.wrapping_sub(1) },
            direction::Direction::Down => Self { x: self.x, y: self.y.saturating_add(1) },
            direction::Direction::Left => Self { x: self.x.wrapping_sub(1), y: self.y },
            direction::Direction::Right => Self { x: self.x.saturating_add(1), y: self.y },
        }
    }
}
