use async_trait::async_trait;
use chrono::NaiveDate;

use crate::domain::{
    Answer, AnswerId, AnswerType, Board, BoardId, Clue, FoundAnswer, Guess, PlayerId,
};

use super::Dictionary;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct GameId(String);

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Game {
    pub board_id: BoardId,
    pub player_id: PlayerId,
    pub active_clue: Option<Clue>,
    pub clue_progress_counter: u32,
    pub found_answer_ids: Vec<AnswerId>,
    pub guesses: Vec<Guess>,
    pub actions: Vec<GameAction>,
}

impl Game {
    pub fn new(board_id: BoardId, player_id: PlayerId) -> Self {
        Game {
            board_id,
            player_id,
            active_clue: None,
            clue_progress_counter: 0,
            found_answer_ids: vec![],
            guesses: vec![],
            actions: vec![],
        }
    }

    pub fn make_guess(
        &mut self,
        guess: Guess,
        board: &Board,
        dictionary: &impl Dictionary,
    ) -> Result<GuessSuccess, GuessFailure> {
        if self.guesses.contains(&guess) {
            return Err(GuessFailure::AlreadyGuessed);
        }

        let word = board
            .get_word(&guess.positions)
            .ok_or(GuessFailure::OutOfBounds)?;

        let response = match board.guess_is_answer(&guess) {
            FoundAnswer::NotAnswer => {
                if dictionary.contains_word(&word) {
                    self.clue_progress_counter += 1;
                    Ok(GuessSuccess::GainedClue(self.clue_progress_counter))
                } else {
                    Err(GuessFailure::NotRealWord)
                }
            }
            FoundAnswer::Found(answer) => {
                self.found_answer(answer.clone());
                Ok(GuessSuccess::FoundAnswer(answer))
            }
        };

        self.guesses.push(guess);
        response
    }

    fn found_answer(&mut self, found_answer: Answer) {
        match found_answer.answer_type {
            AnswerType::Normal => self.actions.push(GameAction::NormalAnswerFound),
            AnswerType::Spangram => self.actions.push(GameAction::SpanogramFound),
        }
        self.found_answer_ids.push(found_answer.id);
    }

    pub fn redeem_clue(&mut self, board: &Board) -> Result<(), RedeemClueFailure> {
        if self.clue_progress_counter < 3 {
            return Err(RedeemClueFailure::NotEnoughClueProgress);
        }
        let clue = board
            .get_next_clue(&self.found_answer_ids)
            .ok_or(RedeemClueFailure::CouldNotFindClue)?;

        self.actions.push(GameAction::Clue);
        self.clue_progress_counter -= 3;
        self.active_clue = Some(clue.clone());

        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum RedeemClueFailure {
    NotEnoughClueProgress,
    CouldNotFindClue,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum GuessSuccess {
    FoundAnswer(Answer),
    GainedClue(u32),
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum GuessFailure {
    AlreadyGuessed,
    OutOfBounds,
    NotRealWord,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum GameAction {
    Clue,
    NormalAnswerFound,
    SpanogramFound,
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct GameHistory(Vec<GameAction>);

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum GameState {
    NotStarted,
    InProgress(Game),
    Finished(Vec<GameAction>),
}

#[async_trait]
pub trait GameRepository {
    async fn by_player_and_board(&self, player_id: &PlayerId, board_id: &BoardId) -> Option<Game>;
    async fn by_id(&self, id: &GameId) -> Option<Game>;
}
