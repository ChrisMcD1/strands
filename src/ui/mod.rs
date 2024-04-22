use itertools::Itertools;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Padding, Paragraph},
};
use unicode_width::UnicodeWidthStr;

use crate::domain;

pub struct Board {
    pub tiles: Tiles,
    pub theme: String,
}

pub struct Tiles(Vec<Vec<char>>);

impl Tiles {
    pub fn new(tiles: Vec<Vec<char>>) -> Self {
        Self(tiles)
    }
    pub fn width(&self) -> usize {
        self.0.first().unwrap().len()
    }
    pub fn height(&self) -> usize {
        self.0.len()
    }
}

impl StatefulWidget for &Tiles {
    type State = domain::Position;

    fn render(self, area: Rect, buf: &mut Buffer, highlighted_position: &mut Self::State) {
        let horizontal_layout =
            Layout::vertical(vec![
                Constraint::Ratio(1, self.height().try_into().unwrap());
                self.height()
            ]);
        let vertical_layout =
            Layout::horizontal(vec![
                Constraint::Ratio(1, self.width().try_into().unwrap());
                self.width()
            ]);

        let cells = horizontal_layout
            .split(area)
            .into_iter()
            .map(|&row| vertical_layout.split(row).to_vec())
            .collect_vec();

        for (i, row) in cells.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                let tile = self.0.get(i).unwrap().get(j).unwrap();
                let block = if highlighted_position.row == i.try_into().unwrap()
                    && highlighted_position.col == j.try_into().unwrap()
                {
                    Block::default()
                        .borders(Borders::ALL)
                        .padding(Padding::new(0, 0, ((cell.height) / 2) - 1, 0))
                        .border_style(Color::Yellow)
                } else {
                    Block::default().padding(Padding::new(0, 0, ((cell.height) / 2) - 1, 0))
                };
                Paragraph::new(Text::raw(tile.to_string()))
                    .block(block)
                    .alignment(Alignment::Center)
                    .render(*cell, buf)
            }
        }
    }
}

impl StatefulWidget for &Board {
    type State = domain::Position;

    fn render(self, area: Rect, buf: &mut Buffer, position: &mut Self::State) {
        // let highlight_symbol = ">";
        // let blank_symbol = " ".repeat(highlight_symbol.width());
        let vertical_split = Layout::horizontal([Constraint::Percentage(50); 2]);
        let [theme_area, tile_area] = vertical_split.areas(area);
        self.tiles.render(tile_area, buf, position);

        Paragraph::new(Text::raw(self.theme.clone()))
            .block(
                Block::default()
                    .title("TODAY'S THEME")
                    .borders(Borders::ALL),
            )
            .alignment(Alignment::Center)
            .render(theme_area, buf);
    }
}
