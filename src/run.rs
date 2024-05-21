use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use crate::handler;

pub fn run() {
    enable_raw_mode().unwrap();
    let mut stdout = std::io::stdout();
    execute!(stdout, EnableMouseCapture).unwrap();

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    // handler::loading::loading(&mut terminal);
    handler::top_page::top_page(&mut terminal);

    disable_raw_mode().unwrap();
    execute!(terminal.backend_mut(), DisableMouseCapture).unwrap();
    terminal.show_cursor().unwrap();
}
