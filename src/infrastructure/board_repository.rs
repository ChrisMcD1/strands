use crate::domain::*;
use async_trait::async_trait;
use chrono::NaiveDate;
use std::collections::HashMap;

pub struct InMemoryBoardRepository(HashMap<BoardId, Board>);

#[async_trait]
impl BoardRepository for InMemoryBoardRepository {
    async fn by_id(&self, id: &BoardId) -> Option<Board> {
        self.0.get(id).cloned()
    }
    async fn by_date(&self, _date: &NaiveDate) -> Option<Board> {
        // cheeky way to find the "first" element. Fix at some point
        self.0.values().find(|_| true).cloned()
    }
}
