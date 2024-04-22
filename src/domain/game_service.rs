use std::{collections::HashMap, sync::Arc};

use chrono::NaiveDate;

use super::{BoardId, BoardRepository, GameRepository, PlayerId, Position};

pub struct GameService {
    game_repository: Arc<dyn GameRepository>,
    board_repository: Arc<dyn BoardRepository>,
}

pub struct GameDto {
    print_date: NaiveDate,
    board_id: BoardId,
    editor: String,
    spangram: String,
    clue: String,
    starting_board: Vec<String>,
    solutions: Vec<String>,
    theme_coords: HashMap<String, Vec<Position>>,
}

impl GameService {
    pub async fn build_real_response(
        &self,
        date: &NaiveDate,
        player_id: &PlayerId,
    ) -> Option<GameDto> {
        let board = self.board_repository.by_date(date).await?;
        let game = self
            .game_repository
            .by_player_and_board(player_id, &board.id)
            .await?;
        Some(GameDto {
            print_date: date.clone(),
            board_id: board.id.clone(),
            editor: board.editor.clone(),
            spangram: board
                .get_word(&board.spangram().positions)
                .expect("Should have a word"),
            clue: board.clue.clone(),
            starting_board: board.tiles,
            solutions: vec![],
            theme_coords: board
                .answers
                .iter()
                .map(|a| (a.word.clone(), a.positions.inner_value()))
                .collect(),
        })
    }
}
