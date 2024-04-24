use crate::domain::*;
use async_trait::async_trait;
use std::collections::HashMap;

pub struct InMemoryGameRepository(HashMap<GameId, Game>);

#[async_trait]
impl GameRepository for InMemoryGameRepository {
    async fn by_id(&self, id: &GameId) -> Option<Game> {
        self.0.get(id).cloned()
    }
    async fn by_player_and_board(&self, player_id: &PlayerId, board_id: &BoardId) -> Option<Game> {
        self.0
            .values()
            .find(|game| &game.player_id == player_id && &game.board_id == board_id)
            .cloned()
    }
}
