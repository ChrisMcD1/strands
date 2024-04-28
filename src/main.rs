pub mod adapter;
pub mod domain;
pub mod infrastructure;
pub mod ui;

use adapter::{NYTBoardDto, NytClient};
use chrono::prelude::*;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use infrastructure::HttpNytClient;
use ratatui::prelude::*;
use std::io::{self, stdout};
use ui::Board;

#[tokio::main]
async fn main() -> io::Result<()> {
    let date = Local::now();
    let nyt_client = HttpNytClient;
    let http_response = nyt_client.by_date(&date.date_naive()).await;

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    // let domain_board = domain::Board::from_string(
    //     domain::BoardId("123".to_string()),
    //     httpResponse.editor,
    //     httpResponse.clue,
    //     answers,
    //     tiles,
    // );
    let board = Board {
        tiles: domain::Tiles::from_strings(&http_response.starting_board).into(),
        theme: http_response.clue,
    };
    let mut app = App::new(board);
    app.run(&mut terminal)?;

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

struct App {
    board: Board,
    highlighted: domain::Position,
    should_quit: bool,
}

impl App {
    pub fn new(board: Board) -> Self {
        Self {
            board,
            highlighted: domain::Position { row: 0, col: 0 },
            should_quit: false,
        }
    }

    fn run(&mut self, terminal: &mut Terminal<impl Backend>) -> io::Result<()> {
        while !self.should_quit {
            self.render(terminal);
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    self.handle_keypress(key)
                }
            }
        }
        Ok(())
    }

    fn handle_keypress(&mut self, key: KeyEvent) {
        use KeyCode::*;
        match key.code {
            Char('q') | Esc => self.should_quit = true,
            Char('h') | Left => self.highlighted = self.highlighted.clone().left(),
            Char('l') | Right => self.highlighted = self.highlighted.clone().right(),
            Char('j') | Down => self.highlighted = self.highlighted.clone().down(),
            Char('k') | Up => self.highlighted = self.highlighted.clone().up(),
            _ => {}
        }
    }

    fn render(&mut self, terminal: &mut Terminal<impl Backend>) {
        let _ = terminal.draw(|frame| {
            frame.render_stateful_widget(&self.board, frame.size(), &mut self.highlighted)
        });
    }
}

#[cfg(test)]
mod test {
    use chrono::NaiveDate;

    use crate::{domain::*, test_fixtures::*};

    #[test]
    fn finds_answer() {
        let tiles = ContiguousPositions::new(vec![
            Position::new(0, 0),
            Position::new(0, 1),
            Position::new(0, 2),
            Position::new(0, 3),
            Position::new(0, 4),
        ])
        .unwrap();
        let answer = Answer::new(AnswerId::new(1), AnswerType::Normal, tiles.clone(), 1);
        let answers = vec![answer.clone()];
        let board = Board::from_string(
            BoardId::new(123),
            "Chris".to_string(),
            "Try This".to_string(),
            NaiveDate::from_ymd_opt(2024, 4, 24).unwrap(),
            answers,
            &[
                "H".to_string(),
                "e".to_string(),
                "l".to_string(),
                "l".to_string(),
                "o".to_string(),
            ],
        )
        .unwrap();
        let guess = Guess::new(tiles).unwrap();

        let found_answer = board.guess_is_answer(&guess);

        assert_eq!(found_answer, FoundAnswer::Found(answer))
    }

    #[test]
    fn says_wrong_guess_is_not_answer() {
        let answer = Answer::new(
            AnswerId::new(1),
            AnswerType::Normal,
            ContiguousPositions::new(vec![
                Position::new(0, 0),
                Position::new(0, 1),
                Position::new(0, 2),
                Position::new(0, 3),
                Position::new(0, 4),
            ])
            .unwrap(),
            1,
        );
        let answers = vec![answer.clone()];
        let board = Board::from_string(
            BoardId::new(123),
            "Chris".to_string(),
            "Try This".to_string(),
            NaiveDate::from_ymd_opt(2024, 4, 24).unwrap(),
            answers,
            &[
                "H".to_string(),
                "e".to_string(),
                "l".to_string(),
                "l".to_string(),
                "o".to_string(),
            ],
        )
        .unwrap();
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
    fn using_sample_board() {
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
    fn gives_credit_for_real_word() {
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
    fn doesnt_give_credit_for_duplicate_guesses() {
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
    fn get_clue_after_3_words() {
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
    fn cannot_redeem_clue_on_fresh_game() {
        let board = sample_board();
        let mut game = sample_game();

        let not_a_clue = game.redeem_clue(&board);

        assert_eq!(not_a_clue, Err(RedeemClueFailure::NotEnoughClueProgress))
    }
}

#[cfg(test)]
mod test_fixtures {
    use chrono::NaiveDate;

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
            AnswerId::new(1),
            AnswerType::Spangram,
            ContiguousPositions::new(vec![
                Position::new(0, 0),
                Position::new(0, 1),
                Position::new(0, 2),
                Position::new(0, 3),
                Position::new(0, 4),
            ])
            .unwrap(),
            1,
        )
    }

    pub fn sample_game() -> Game {
        Game::new(BoardId::new(123), PlayerId::new("chrismcdonnell"))
    }

    pub fn sample_board() -> Board {
        let board_id = BoardId::new(123);
        let tiles = vec![
            "hello".to_string(),
            "world".to_string(),
            "thisi".to_string(),
            "fooba".to_string(),
            "rbazo".to_string(),
        ];
        let answers = vec![spanogram_answer()];
        let clue = "Try This".to_string();
        let editor = "Chris".to_string();
        let print_date = NaiveDate::from_ymd_opt(2024, 4, 24).unwrap();

        Board::from_string(board_id, editor, clue, print_date, answers, &tiles).unwrap()
    }
}
