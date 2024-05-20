use crate::ui;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{backend::CrosstermBackend, widgets::List};
use ratatui::{widgets::ListState, Terminal};

pub fn run() {
    enable_raw_mode().unwrap();
    let mut stdout = std::io::stdout();
    execute!(stdout, EnableMouseCapture).unwrap();

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.clear().unwrap();

    let mut focus_idx = 0;
    let mut list_state = ListState::default();
    let mut lis_len = 0;
    list_state.select(Some(0));
    loop {
        terminal
            .draw(|f| lis_len = ui::top_page::top_page(f, focus_idx, &mut list_state))
            .unwrap();
        if let Event::Key(key) = event::read().unwrap() {
            match (key.code, key.kind) {
                (KeyCode::Char('q'), KeyEventKind::Press) => break,
                (KeyCode::Char('l'), KeyEventKind::Press)
                | (KeyCode::Right, KeyEventKind::Press) => {
                    focus_idx = (focus_idx + 1) % 3;
                    list_state = ListState::default();
                    list_state.select(Some(0));
                }
                (KeyCode::Char('h'), KeyEventKind::Press)
                | (KeyCode::Left, KeyEventKind::Press) => {
                    focus_idx = (focus_idx + 2) % 3;
                    list_state = ListState::default();
                    list_state.select(Some(0));
                }
                (KeyCode::Char('j'), KeyEventKind::Press)
                | (KeyCode::Down, KeyEventKind::Press) => {
                    list_state.select(Some((list_state.selected().unwrap() + 1) % lis_len));
                }
                (KeyCode::Char('k'), KeyEventKind::Press) | (KeyCode::Up, KeyEventKind::Press) => {
                    list_state.select(Some(
                        (list_state.selected().unwrap() + lis_len - 1) % lis_len,
                    ));
                }
                _ => {}
            }
        }
    }

    disable_raw_mode().unwrap();
    execute!(terminal.backend_mut(), DisableMouseCapture).unwrap();
    terminal.show_cursor().unwrap();
}
