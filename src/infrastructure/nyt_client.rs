use crate::adapter::{NYTBoardDto, NytClient};
use async_trait::async_trait;
use chrono::prelude::*;

pub struct HttpNytClient;

#[async_trait]
impl NytClient for HttpNytClient {
    async fn by_date(&self, date: &NaiveDate) -> NYTBoardDto {
        let url = format!(
            "https://www.nytimes.com/games-assets/strands/{}.json",
            date.format("%Y-%m-%d")
        );
        let http_response = reqwest::get(url)
            .await
            .unwrap()
            .json::<NYTBoardDto>()
            .await
            .unwrap();
        http_response
    }
}
