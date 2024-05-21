use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
};

pub fn loading(f: &mut ratatui::Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(5), Constraint::Percentage(95)].as_ref())
        .split(f.size());

    let top_bar = Paragraph::new("AtCoder TUI")
        .block(Block::default().borders(Borders::BOTTOM))
        .alignment(ratatui::layout::Alignment::Left);
    f.render_widget(top_bar, chunks[0]);

    let loading_window = Paragraph::new("Now Loading...")
        .block(Block::default().borders(Borders::ALL))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(loading_window, chunks[1]);
}
