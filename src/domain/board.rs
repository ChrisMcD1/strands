use std::collections::{HashMap, HashSet};

use async_trait::async_trait;
use chrono::NaiveDate;

use crate::domain::{Answer, AnswerId, Clue, Guess, Position};

use super::{AnswerType, ContiguousPositions};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct BoardId(String);

impl BoardId {
    pub fn new(str: &str) -> Self {
        BoardId(str.to_string())
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Board {
    pub id: BoardId,
    pub editor: String,
    pub clue: String,
    pub answers: Vec<Answer>,
    pub tiles_map: HashMap<Position, char>,
    pub tiles: Vec<String>,
    dimensions: Dimensions,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dimensions {
    pub width: usize,
    pub height: usize,
}

impl Board {
    fn new(
        id: BoardId,
        answers: Vec<Answer>,
        tiles_map: HashMap<Position, char>,
        tiles: &[&str],
        dimensions: Dimensions,
    ) -> Result<Self, InvalidBoard> {
        let answer_tiles_set: HashSet<Position> = answers
            .iter()
            .flat_map(|a| a.positions.inner_value())
            .collect();
        let actual_tiles_set: HashSet<Position> = tiles_map.keys().cloned().collect();

        if answer_tiles_set.eq(&actual_tiles_set) {
            return Err(InvalidBoard::AnswersDontCoverAllTiles);
        }

        Ok(Board {
            id,
            clue: "Do well!".to_string(),
            editor: "FooBar".to_string(),
            answers,
            tiles_map,
            tiles: tiles.iter().map(|s| s.to_string()).collect(),
            dimensions,
        })
    }

    pub fn from_string(
        id: BoardId,
        answers: Vec<Answer>,
        tiles: &[&str],
    ) -> Result<Self, InvalidBoard> {
        let height = tiles.len();
        let width = tiles
            .first()
            .map(|row| row.len())
            .ok_or(InvalidBoard::InconsistentDimensions)?;
        if tiles
            .iter()
            .map(|row| row.len())
            .any(|row_width| row_width != width)
        {
            return Err(InvalidBoard::InconsistentDimensions);
        }

        let tiles_map = tiles
            .into_iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.chars()
                    .enumerate()
                    .map(move |(j, letter)| {
                        (
                            Position::new(i.try_into().unwrap(), j.try_into().unwrap()),
                            letter,
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        Board::new(id, answers, tiles_map, tiles, Dimensions { width, height })
    }

    pub fn spangram(&self) -> &Answer {
        self.answers
            .iter()
            .find(|a| a.answer_type == AnswerType::Spangram)
            .expect("Should be impossible to construct a board without a spangram.")
    }

    pub fn hello(&self) -> String {
        "Hello".to_string()
    }

    pub fn guess_is_answer(&self, guess: &Guess) -> FoundAnswer {
        let answer = self
            .answers
            .iter()
            .find(|answer| answer.matches_guess(guess));
        match answer {
            Some(found_answer) => FoundAnswer::Found(found_answer.clone()),
            None => FoundAnswer::NotAnswer,
        }
    }

    pub fn get_next_clue(&self, found_answer_ids: &[AnswerId]) -> Option<Clue> {
        self.answers
            .iter()
            .find(|answer| !found_answer_ids.iter().any(|found| found == &answer.id))
            .map(|next_answer| next_answer.clone().into())
    }

    pub fn get_word(&self, positions: &ContiguousPositions) -> Option<String> {
        positions
            .iter()
            .map(|position| self.tiles_map.get(position))
            .collect::<Option<Vec<&char>>>()
            .map(|chars| chars.into_iter().collect::<String>())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum InvalidBoard {
    InconsistentDimensions,
    AnswersDontCoverAllTiles,
}

#[derive(Debug, PartialEq, Eq)]
pub enum FoundAnswer {
    NotAnswer,
    Found(Answer),
}

#[async_trait]
pub trait BoardRepository {
    async fn by_date(&self, date: &NaiveDate) -> Option<Board>;
    async fn by_id(&self, id: &BoardId) -> Option<Board>;
}
