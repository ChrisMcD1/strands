use std::collections::HashSet;

use chrono::NaiveDate;
use itertools::Itertools;

use crate::{
    domain::{Answer, AnswerId, Clue, Guess, Position},
    NYTBoardDto,
};

use super::{AnswerType, ContiguousPositions};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct BoardId(pub String);

impl BoardId {
    pub fn new(str: &str) -> Self {
        BoardId(str.to_string())
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Tile(pub char);

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Tiles(pub Vec<Vec<Tile>>);

impl Tiles {
    pub fn new(tiles: Vec<Vec<Tile>>) -> Self {
        Self(tiles)
    }
    pub fn from_strings(strings: &[String]) -> Self {
        let tiles: Vec<Vec<Tile>> = strings
            .into_iter()
            .map(|row| row.chars().map(move |letter| Tile(letter)).collect_vec())
            .collect_vec();

        Tiles::new(tiles)
    }
    pub fn at_position(&self, position: &Position) -> Option<Tile> {
        let row: usize = position.row.try_into().ok()?;
        let col: usize = position.row.try_into().ok()?;

        self.0.get(row)?.get(col).copied()
    }
    pub fn all_positions(&self) -> Vec<Position> {
        self.0
            .iter()
            .enumerate()
            .flat_map(move |(i, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(j, _)| Position::from_usize(i, j))
            })
            .collect_vec()
    }
    pub fn get_word(&self, positions: &ContiguousPositions) -> Option<String> {
        positions
            .iter()
            .map(|position| self.at_position(position).map(|tile| tile.0))
            .collect::<Option<Vec<char>>>()
            .map(|chars| chars.into_iter().collect::<String>())
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Board {
    pub id: BoardId,
    pub print_date: NaiveDate,
    pub editor: String,
    pub clue: String,
    pub answers: Vec<Answer>,
    pub tiles: Tiles,
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
        editor: String,
        clue: String,
        print_date: NaiveDate,
        answers: Vec<Answer>,
        tiles: Tiles,
        dimensions: Dimensions,
    ) -> Result<Self, InvalidBoard> {
        let answer_tiles_set: HashSet<Position> = answers
            .iter()
            .flat_map(|a| a.positions.inner_value())
            .collect();
        let actual_tiles_set: HashSet<Position> = tiles.all_positions().into_iter().collect();

        if answer_tiles_set.eq(&actual_tiles_set) {
            return Err(InvalidBoard::AnswersDontCoverAllTiles);
        }

        Ok(Board {
            id,
            clue,
            editor,
            print_date,
            answers,
            tiles,
            dimensions,
        })
    }

    pub fn from_string(
        id: BoardId,
        editor: String,
        clue: String,
        print_date: NaiveDate,
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

        let tiles: Tiles = Tiles::from_strings(&tiles.iter().map(|t| t.to_string()).collect_vec());

        Board::new(
            id,
            editor,
            clue,
            print_date,
            answers,
            tiles,
            Dimensions { width, height },
        )
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
        self.tiles.get_word(positions)
    }
}

impl From<NYTBoardDto> for Board {
    fn from(nyt: NYTBoardDto) -> Self {
        todo!()
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
