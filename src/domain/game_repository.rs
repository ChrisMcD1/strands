use crate::domain::*;
use async_trait::async_trait;

#[async_trait]
pub trait GameRepository {
    async fn by_player_and_board(&self, player_id: &PlayerId, board_id: &BoardId) -> Option<Game>;
    async fn by_id(&self, id: &GameId) -> Option<Game>;
}
