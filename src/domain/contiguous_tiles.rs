use std::{collections::HashSet, slice::Iter};

use itertools::Itertools;

use super::Position;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContiguousPositions(Vec<Position>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreateContiguousTilesError {
    Disconnected,
    HasDuplicates,
}

impl ContiguousPositions {
    pub fn new(positions: Vec<Position>) -> Result<Self, CreateContiguousTilesError> {
        if Self::has_duplicates(&positions) {
            return Err(CreateContiguousTilesError::HasDuplicates);
        }

        if Self::breaks_continuity(&positions) {
            return Err(CreateContiguousTilesError::Disconnected);
        }

        Ok(Self(positions))
    }

    pub fn inner_value(&self) -> Vec<Position> {
        self.0.clone()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Position> {
        self.0.iter()
    }

    fn breaks_continuity(positions: &[Position]) -> bool {
        positions
            .iter()
            .tuples()
            .any(|(first, second)| !first.is_adjacent_to(second))
    }

    fn has_duplicates(positions: &[Position]) -> bool {
        HashSet::<&Position>::from_iter(positions.into_iter()).len() != positions.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn errors_for_duplicates() -> () {
        let duplicate_positions =
            ContiguousPositions::new(vec![Position::new(0, 0), Position::new(0, 0)]);

        assert_eq!(
            duplicate_positions,
            Err(CreateContiguousTilesError::HasDuplicates)
        )
    }

    #[test]
    fn errors_for_disconnected() -> () {
        let disconnected_positions =
            ContiguousPositions::new(vec![Position::new(0, 0), Position::new(0, 2)]);

        assert_eq!(
            disconnected_positions,
            Err(CreateContiguousTilesError::Disconnected)
        )
    }
}
