//! Encapsulates the state of a piet program

use std::fmt;
use std::ops::{Add, AddAssign};

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub left: u32,
    pub top: u32,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.left, self.top)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Add for Direction {
    type Output = Direction;

    fn add(self, rhs: Direction) -> Direction {
        match rhs {
            // 90deg turn
            Direction::Right => {
                match self {
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Up => Direction::Right,
                }
            }
            // Opposite direction
            Direction::Down => {
                match self {
                    Direction::Right => Direction::Left,
                    Direction::Down => Direction::Up,
                    Direction::Left => Direction::Right,
                    Direction::Up => Direction::Down,
                }
            }
            // -90deg turn
            Direction::Left => {
                match self {
                    Direction::Right => Direction::Up,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Down,
                    Direction::Up => Direction::Left,
                }
            }
            Direction::Up => self,
        }
    }
}

impl AddAssign for Direction {
    fn add_assign(&mut self, other: Direction) {
        *self = *self + other;
    }
}

#[derive(Debug)]
pub struct State {
    pub stack: Vec<i32>,
    pub pos: Position,
    pub choosing_codel: bool,
    dp: Direction,
    cc: Direction,
}

impl State {
    pub fn new() -> State {
        State {
            dp: Direction::Right,
            cc: Direction::Left,
            stack: vec![],
            pos: Position { left: 0, top: 0 },
            choosing_codel: false,
        }
    }

    pub fn dp(&self) -> Direction {
        self.dp
    }

    pub fn cc(&self) -> Direction {
        self.cc
    }

    pub fn codel_direction(&self) -> Direction {
        self.cc + self.dp
    }

    pub fn pos(&self) -> Position {
        self.pos
    }

    pub fn peek_pos(&self) -> Position {
        let mut newpos = self.pos;
        let shift_dir = match self.choosing_codel {
            true => self.codel_direction(),
            false => self.dp,
        };

        match shift_dir {
            Direction::Left => {
                newpos.left -= 1;
            }
            Direction::Right => {
                newpos.left += 1;
            }
            Direction::Up => {
                newpos.top -= 1;
            }
            Direction::Down => {
                newpos.top += 1;
            }
        }
        newpos
    }

    pub fn advance(&mut self) {
        let pos = self.pos;
        self.pos = self.peek_pos();
        println!("Advance from {} to {}", pos, self.pos);
    }

    pub fn rot_clockwise(&mut self) {
        println!("Rotate clockwise to {:?}", self.dp + Direction::Right);
        self.dp += Direction::Right;
    }

    pub fn rot_counterclockwise(&mut self) {
        println!("Rotate counterclockwise to {:?}", self.dp + Direction::Left);
        self.dp += Direction::Left;
    }

    pub fn toggle_cc(&mut self) {
        self.cc = match self.cc {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            _ => panic!("self.cc was neither left nor right. This should NEVER happen"),
        };
    }
}
