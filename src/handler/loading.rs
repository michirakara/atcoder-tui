use crate::ui;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::Terminal;

pub fn loading<T: ratatui::backend::Backend>(terminal: &mut Terminal<T>) {
    terminal.clear().unwrap();
    loop {
        terminal.draw(ui::loading::loading).unwrap();
        if let Event::Key(key) = event::read().unwrap() {
            match (key.code, key.kind) {
                (KeyCode::Char('q'), KeyEventKind::Press) => break,
                _ => {}
            }
        }
    }
}
