use crate::ui;
use ratatui::Terminal;

pub fn loading<T: ratatui::backend::Backend>(terminal: &mut Terminal<T>) {
    terminal.clear().unwrap();
    terminal.draw(ui::loading::loading).unwrap();
}
