use std::{collections::HashMap, sync::Arc};

use chrono::NaiveDate;

use crate::ui;

use super::{BoardId, BoardRepository, GameRepository, PlayerId, Position};

pub struct GameService {
    game_repository: Arc<dyn GameRepository>,
    board_repository: Arc<dyn BoardRepository>,
}

impl GameService {
    pub async fn build_real_response(
        &self,
        date: &NaiveDate,
        player_id: &PlayerId,
    ) -> Option<ui::Board> {
        let board = self.board_repository.by_date(date).await?;
        let game = self
            .game_repository
            .by_player_and_board(player_id, &board.id)
            .await?;
        Some(ui::Board {
            tiles: board.tiles.into(),
            theme: board.clue,
        })
    }
}
