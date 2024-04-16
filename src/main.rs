pub mod domain;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use crate::{domain::*, test_fixtures::*};

    #[test]
    fn finds_answer() -> () {
        let tiles = ContiguousPositions::new(vec![
            Position::new(0, 0),
            Position::new(0, 1),
            Position::new(0, 2),
            Position::new(0, 3),
            Position::new(0, 4),
        ])
        .unwrap();
        let answer = Answer::new(
            AnswerId::new("abc"),
            AnswerType::Normal,
            tiles.clone(),
            "Hello".to_string(),
            1,
        );
        let answers = vec![answer.clone()];
        let board = Board::new(BoardId::new("123"), answers, &vec![]).unwrap();
        let guess = Guess::new(tiles).unwrap();

        let found_answer = board.guess_is_answer(&guess);

        assert_eq!(found_answer, FoundAnswer::Found(answer))
    }

    #[test]
    fn says_wrong_guess_is_not_answer() -> () {
        let answer = Answer::new(
            AnswerId::new("abc"),
            AnswerType::Normal,
            ContiguousPositions::new(vec![
                Position::new(0, 0),
                Position::new(0, 1),
                Position::new(0, 2),
                Position::new(0, 3),
                Position::new(0, 4),
            ])
            .unwrap(),
            "Hello".to_string(),
            1,
        );
        let answers = vec![answer.clone()];
        let board = Board::new(BoardId::new("123"), answers, &vec![]).unwrap();
        let guess = Guess::new(
            ContiguousPositions::new(vec![
                Position::new(1, 0),
                Position::new(0, 1),
                Position::new(0, 2),
                Position::new(0, 3),
                Position::new(0, 4),
            ])
            .unwrap(),
        )
        .unwrap();

        let found_answer = board.guess_is_answer(&guess);

        assert_eq!(found_answer, FoundAnswer::NotAnswer)
    }

    #[test]
    fn using_sample_board() -> () {
        let board = sample_board();
        let mut game = sample_game();
        let dictionary = AlwaysContainsDictionary;
        let guess = Guess::new(
            ContiguousPositions::new(vec![
                Position::new(0, 0),
                Position::new(0, 1),
                Position::new(0, 2),
                Position::new(0, 3),
                Position::new(0, 4),
            ])
            .unwrap(),
        )
        .unwrap();

        let answer = game.make_guess(guess, &board, &dictionary);

        assert_eq!(answer, Ok(GuessSuccess::FoundAnswer(spanogram_answer())))
    }

    #[test]
    fn gives_credit_for_real_word() -> () {
        let board = sample_board();
        let mut game = sample_game();
        let dictionary = AlwaysContainsDictionary;
        let guess = Guess::new(
            ContiguousPositions::new(vec![
                Position::new(0, 0),
                Position::new(0, 1),
                Position::new(0, 2),
                Position::new(0, 3),
            ])
            .unwrap(),
        )
        .unwrap();

        let answer = game.make_guess(guess, &board, &dictionary);

        assert_eq!(answer, Ok(GuessSuccess::GainedClue(1)))
    }

    #[test]
    fn doesnt_give_credit_for_duplicate_guesses() -> () {
        let board = sample_board();
        let mut game = sample_game();
        let dictionary = AlwaysContainsDictionary;
        let guess = Guess::new(
            ContiguousPositions::new(vec![
                Position::new(0, 0),
                Position::new(0, 1),
                Position::new(0, 2),
                Position::new(0, 3),
            ])
            .unwrap(),
        )
        .unwrap();

        let _first_guess = game.make_guess(guess.clone(), &board, &dictionary);
        let duplicate_guess_response = game.make_guess(guess, &board, &dictionary);

        assert_eq!(duplicate_guess_response, Err(GuessFailure::AlreadyGuessed))
    }

    #[test]
    fn get_clue_after_3_words() -> () {
        let board = sample_board();
        let mut game = sample_game();
        let guess_1 = Guess::new(
            ContiguousPositions::new(vec![
                Position::new(0, 0),
                Position::new(0, 1),
                Position::new(0, 2),
                Position::new(0, 3),
            ])
            .unwrap(),
        )
        .unwrap();
        let guess_2 = Guess::new(
            ContiguousPositions::new(vec![
                Position::new(0, 1),
                Position::new(0, 2),
                Position::new(0, 3),
                Position::new(1, 3),
            ])
            .unwrap(),
        )
        .unwrap();
        let guess_3 = Guess::new(
            ContiguousPositions::new(vec![
                Position::new(0, 1),
                Position::new(0, 2),
                Position::new(1, 3),
                Position::new(1, 4),
            ])
            .unwrap(),
        )
        .unwrap();
        let dictionary = AlwaysContainsDictionary;

        let _ = game.make_guess(guess_1, &board, &dictionary);
        let _ = game.make_guess(guess_2, &board, &dictionary);
        let _ = game.make_guess(guess_3, &board, &dictionary);

        let successfully_made_clue = game.redeem_clue(&board);

        assert_eq!(successfully_made_clue, Ok(()));
        assert_eq!(game.active_clue, Some(spanogram_answer().into()));
    }

    #[test]
    fn cannot_redeem_clue_on_fresh_game() -> () {
        let board = sample_board();
        let mut game = sample_game();

        let not_a_clue = game.redeem_clue(&board);

        assert_eq!(not_a_clue, Err(RedeemClueFailure::NotEnoughClueProgress))
    }
}

#[cfg(test)]
mod test_fixtures {
    use crate::domain::*;

    pub struct AlwaysContainsDictionary;
    impl Dictionary for AlwaysContainsDictionary {
        fn contains_word(&self, _word: &str) -> bool {
            true
        }
    }

    pub struct NeverContainsDictionary;
    impl Dictionary for NeverContainsDictionary {
        fn contains_word(&self, _word: &str) -> bool {
            false
        }
    }

    pub fn spanogram_answer() -> Answer {
        Answer::new(
            AnswerId::new("abc"),
            AnswerType::Spanogram,
            ContiguousPositions::new(vec![
                Position::new(0, 0),
                Position::new(0, 1),
                Position::new(0, 2),
                Position::new(0, 3),
                Position::new(0, 4),
            ])
            .unwrap(),
            "Hello".to_string(),
            1,
        )
    }

    pub fn sample_game() -> Game {
        Game::new(BoardId::new("123"), PlayerId::new("chrismcdonnell"))
    }

    pub fn sample_board() -> Board {
        let board_id = BoardId::new("123");
        let tiles = vec!["hello", "world", "thisi", "fooba", "rbazo"];
        let answers = vec![spanogram_answer()];
        Board::new(board_id, answers, &tiles).unwrap()
    }
}
