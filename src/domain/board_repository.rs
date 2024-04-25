use crate::domain::*;
use async_trait::async_trait;
use chrono::NaiveDate;

#[async_trait]
pub trait BoardRepository {
    async fn by_date(&self, date: &NaiveDate) -> Option<Board>;
    async fn by_id(&self, id: &BoardId) -> Option<Board>;
    async fn insert(&self, board: Board) -> ();
}
