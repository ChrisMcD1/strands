use super::ContiguousPositions;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Guess {
    pub positions: ContiguousPositions,
}

impl Guess {
    pub fn new(positions: ContiguousPositions) -> Result<Self, CreateGuessError> {
        if positions.len() < 4 {
            return Err(CreateGuessError::TooShort);
        }
        Ok(Self { positions })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum CreateGuessError {
    TooShort,
}
