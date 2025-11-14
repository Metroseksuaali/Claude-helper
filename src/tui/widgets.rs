// Custom widgets for TUI
// Future implementations can go here

use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders},
};

pub fn create_block(title: &str) -> Block {
    Block::default()
        .borders(Borders::ALL)
        .title(title)
        .style(Style::default().fg(Color::White))
}
