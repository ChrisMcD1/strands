use super::{ContiguousPositions, Guess};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct AnswerId(String);

impl AnswerId {
    pub fn new(str: &str) -> Self {
        AnswerId(str.to_string())
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum AnswerType {
    Normal,
    Spanogram,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Answer {
    pub id: AnswerId,
    pub answer_type: AnswerType,
    pub positions: ContiguousPositions,
    pub word: String,
    pub order: u32,
}

impl Answer {
    pub fn new(
        id: AnswerId,
        answer_type: AnswerType,
        positions: ContiguousPositions,
        word: String,
        order: u32,
    ) -> Self {
        Answer {
            id,
            answer_type,
            positions,
            word,
            order,
        }
    }

    pub fn matches_guess(&self, guess: &Guess) -> bool {
        self.positions == guess.positions
    }
}
