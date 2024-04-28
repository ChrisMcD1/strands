use std::collections::HashMap;

use async_trait::async_trait;
use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionDto(pub usize, pub usize);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NYTBoardDto {
    pub id: u32,
    pub editor: String,
    pub print_date: NaiveDate,
    pub spangram: String,
    pub clue: String,
    pub starting_board: Vec<String>,
    pub solutions: Vec<String>,
    pub theme_coords: HashMap<String, Vec<PositionDto>>,
}

#[async_trait]
pub trait NytClient {
    async fn by_date(&self, date: &NaiveDate) -> NYTBoardDto;
}
