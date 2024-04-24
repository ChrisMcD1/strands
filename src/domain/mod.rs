pub mod answer;
pub mod board;
pub mod board_repository;
pub mod contiguous_tiles;
pub mod game;
pub mod game_repository;
pub mod game_service;
pub mod guess;

use std::char;
use std::collections::HashSet;

pub use self::answer::*;
pub use self::board::*;
pub use self::board_repository::*;
pub use self::contiguous_tiles::*;
pub use self::game::*;
pub use self::game_repository::*;
pub use self::game_service::*;
pub use self::guess::*;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct PlayerId(String);

impl PlayerId {
    pub fn new(str: &str) -> Self {
        PlayerId(str.to_string())
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Position {
    pub row: i32,
    pub col: i32,
}

impl Position {
    pub fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }
    pub fn from_usize(row: usize, col: usize) -> Self {
        Self {
            row: row.try_into().unwrap(),
            col: col.try_into().unwrap(),
        }
    }
    pub fn is_adjacent_to(&self, other: &Position) -> bool {
        (self.row - other.row).abs() <= 1 && (self.col - other.col).abs() <= 1
    }
    pub fn left(self) -> Self {
        Self {
            row: self.row,
            col: self.col - 1,
        }
    }
    pub fn right(self) -> Self {
        Self {
            row: self.row,
            col: self.col + 1,
        }
    }
    pub fn up(self) -> Self {
        Self {
            row: self.row - 1,
            col: self.col,
        }
    }
    pub fn down(self) -> Self {
        Self {
            row: self.row + 1,
            col: self.col,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Tile {
    pub letter: char,
    pub position: Position,
}

impl Tile {
    pub fn new(letter: char, row: i32, col: i32) -> Self {
        Tile {
            letter,
            position: Position { row, col },
        }
    }

    pub fn can_connect_to(&self, other: &Tile) -> bool {
        self.position.is_adjacent_to(&other.position)
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Clue {
    positions: ContiguousPositions,
}

impl From<Answer> for Clue {
    fn from(value: Answer) -> Self {
        Self {
            positions: value.positions,
        }
    }
}

impl Clue {
    pub fn tiles_randomized(&self) -> HashSet<Position> {
        self.positions.inner_value().into_iter().collect()
    }
}

pub trait Dictionary {
    fn contains_word(&self, word: &str) -> bool;
}

pub struct HashSetDictionary(HashSet<String>);

impl Dictionary for HashSetDictionary {
    fn contains_word(&self, word: &str) -> bool {
        self.0.contains(word)
    }
}
