use std::collections::VecDeque;

use crate::models::{direction::Direction, position::Position};

pub struct Snake {
    body: VecDeque<Position>,
    current_direction: Direction,
    next_direction: Direction,
}

impl Snake {
    pub fn new(center: Position) -> Self {
        let mut body = VecDeque::new();
        body.push_back(center);
        body.push_back(Position::new(center.x.saturating_sub(1), center.y));
        body.push_back(Position::new(center.x.saturating_sub(2), center.y));

        Self { body, current_direction: Direction::Right, next_direction: Direction::Right }
    }

    pub fn head(&self) -> Position {
        *self.body.front().expect("Snake must have segments")
    }

    pub fn body(&self) -> &VecDeque<Position> {
        &self.body
    }

    pub fn current_direction(&self) -> Direction {
        self.current_direction
    }

    pub fn next_direction(&self) -> Direction {
        self.next_direction
    }

    pub fn change_direction(&mut self, dir: Direction) {
        if !self.next_direction.is_opposite(dir) {
            self.next_direction = dir;
        }
    }

    pub fn next_head(&self) -> Position {
        self.head().moved(self.next_direction)
    }

    pub fn advance(&mut self, new_head: Position, grow: bool) {
        self.current_direction = self.next_direction;
        self.body.push_front(new_head);
        if !grow {
            self.body.pop_back();
        }
    }

    pub fn intersects_position(&self, pos: Position) -> bool {
        self.body.contains(&pos)
    }

    pub fn self_collides_at(&self, pos: Position, growing: bool) -> bool {
        let check_len = if growing { self.body.len() } else { self.body.len().saturating_sub(1) };

        self.body.iter().take(check_len).any(|&seg| seg == pos)
    }
}
