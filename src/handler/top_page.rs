use crate::backend;
use crate::ui;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{widgets::ListState, Terminal};

pub fn top_page<T: ratatui::backend::Backend>(terminal: &mut Terminal<T>) {
    terminal.clear().unwrap();

    let (active_contests, upcoming_contests, recent_contests) = backend::top_page::get_contests();

    let mut data = ui::top_page::Data {
        focus_idx: 0,
        contests: [
            ui::top_page::ListWithState {
                vec: Some(&active_contests),
                list_state: ListState::default(),
            },
            ui::top_page::ListWithState {
                vec: Some(&upcoming_contests),
                list_state: ListState::default(),
            },
            ui::top_page::ListWithState {
                vec: Some(&recent_contests),
                list_state: ListState::default(),
            },
        ],
    };

    if data.contests[data.focus_idx].vec.unwrap().len() != 0 {
        data.contests[data.focus_idx].list_state.select(Some(0));
    }

    let mut lis_len = data.contests[data.focus_idx].vec.unwrap().len();
    loop {
        terminal
            .draw(|f| ui::top_page::top_page(f, &mut data))
            .unwrap();
        if let Event::Key(key) = event::read().unwrap() {
            match (key.code, key.kind) {
                (KeyCode::Char('q'), KeyEventKind::Press) => break,
                (KeyCode::Char('l'), KeyEventKind::Press)
                | (KeyCode::Right, KeyEventKind::Press) => {
                    data.contests[data.focus_idx].list_state.select(None);
                    data.focus_idx = (data.focus_idx + 1) % 3;
                    lis_len = data.contests[data.focus_idx].vec.unwrap().len();
                    if lis_len != 0 {
                        data.contests[data.focus_idx].list_state.select(Some(0));
                    }
                }
                (KeyCode::Char('h'), KeyEventKind::Press)
                | (KeyCode::Left, KeyEventKind::Press) => {
                    data.contests[data.focus_idx].list_state.select(None);
                    data.focus_idx = (data.focus_idx + 2) % 3;
                    lis_len = data.contests[data.focus_idx].vec.unwrap().len();
                    if lis_len != 0 {
                        data.contests[data.focus_idx].list_state.select(Some(0));
                    }
                }
                (KeyCode::Char('j'), KeyEventKind::Press)
                | (KeyCode::Down, KeyEventKind::Press) => {
                    if lis_len != 0 {
                        data.contests[data.focus_idx].list_state.select(Some(
                            (data.contests[data.focus_idx].list_state.selected().unwrap() + 1)
                                % lis_len,
                        ));
                    }
                }
                (KeyCode::Char('k'), KeyEventKind::Press) | (KeyCode::Up, KeyEventKind::Press) => {
                    if lis_len != 0 {
                        data.contests[data.focus_idx].list_state.select(Some(
                            (data.contests[data.focus_idx].list_state.selected().unwrap()
                                + lis_len
                                - 1)
                                % lis_len,
                        ))
                    };
                }
                _ => {}
            }
        }
    }
}
