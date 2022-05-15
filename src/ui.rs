use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Frame,
};

use crate::app::App;

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // Create main layout
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    let block = Block::default().title("Block").borders(Borders::ALL);
    f.render_widget(block, chunks[0]);
    let block = Block::default().title("Block 2").borders(Borders::ALL);
    f.render_widget(block, chunks[1]);
}
