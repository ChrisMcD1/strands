use crate::domain::*;
use async_trait::async_trait;
use chrono::NaiveDate;
use std::{collections::HashMap, sync::Mutex};

pub struct InMemoryBoardRepository(Mutex<HashMap<BoardId, Board>>);

#[async_trait]
impl BoardRepository for InMemoryBoardRepository {
    async fn by_id(&self, id: &BoardId) -> Option<Board> {
        self.0.lock().map(|m| m.get(id).cloned()).unwrap()
    }

    async fn by_date(&self, date: &NaiveDate) -> Option<Board> {
        // cheeky way to find the "first" element. Fix at some point
        self.0
            .lock()
            .map(|m| m.values().find(|board| &board.print_date == date).cloned())
            .unwrap()
    }

    async fn insert(&self, board: Board) -> () {
        self.0
            .lock()
            .map(|mut m| m.insert(board.id.clone(), board))
            .unwrap();
    }
}
