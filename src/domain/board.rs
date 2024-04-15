use std::collections::HashMap;

use crate::domain::{Answer, AnswerId, Clue, Guess, Position};

use super::ContiguousPositions;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BoardId(String);

impl BoardId {
    pub fn new(str: &str) -> Self {
        BoardId(str.to_string())
    }
}

#[derive(Debug)]
pub struct Board {
    pub id: BoardId,
    pub answers: Vec<Answer>,
    pub tiles: HashMap<Position, char>,
}

impl Board {
    pub fn new(id: BoardId, answers: Vec<Answer>, tiles: &[&str]) -> Result<Self, InvalidBoard> {
        let height = tiles.len();
        if tiles
            .iter()
            .map(|row| row.len())
            .any(|width| width != height)
        {
            return Err(InvalidBoard::InconsistentDimensions);
        }

        let correct_tiles = tiles
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

        Ok(Board {
            id,
            answers,
            tiles: correct_tiles,
        })
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
            .map(|next_answer| next_answer.to_clue())
    }

    pub fn get_word(&self, positions: &ContiguousPositions) -> Option<String> {
        positions
            .iter()
            .map(|position| self.tiles.get(position))
            .collect::<Option<Vec<&char>>>()
            .map(|chars| chars.into_iter().collect::<String>())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum InvalidBoard {
    InconsistentDimensions,
}

#[derive(Debug, PartialEq, Eq)]
pub enum FoundAnswer {
    NotAnswer,
    Found(Answer),
}
