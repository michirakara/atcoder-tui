use crate::backend;
use crate::handler;
use crate::ui;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{widgets::ListState, Terminal};

pub fn contest<T: ratatui::backend::Backend>(terminal: &mut Terminal<T>, contest_path: &str) {
    handler::loading::loading(terminal);

    // TODO
    let (problems, urls) = backend::get_problems::get_problems(contest_path);

    let mut data = ui::contest::Data {
        selection: ui::contest::Selection::Problems,
        problems: ui::contest::ListWithState {
            vec: Some(&problems),
            list_state: ListState::default(),
        },
        statements: ui::contest::StatementWithState {
            statements: vec![None; problems.len()],
            now_idx: Some(0),
            scroll_state: (0, 0),
        },
    };
    data.problems.list_state.select(Some(0));

    terminal.clear().unwrap();

    loop {
        terminal
            .draw(|f| ui::contest::contest(f, &mut data))
            .unwrap();

        if let Event::Key(key) = event::read().unwrap() {
            match (key.code, key.kind) {
                (KeyCode::Char('q'), KeyEventKind::Press) => break,
                (KeyCode::Char('P'), KeyEventKind::Press) => {
                    data.selection = ui::contest::Selection::Problems;
                    // data.problems.list_state.select(Some(0));
                }
                (KeyCode::Char('S'), KeyEventKind::Press) => {
                    data.selection = ui::contest::Selection::Statement;
                }
                (KeyCode::Char('R'), KeyEventKind::Press) => {
                    data.selection = ui::contest::Selection::Submissions;
                }

                (KeyCode::Char('I'), KeyEventKind::Press) => {
                    data.selection = ui::contest::Selection::InputExample;
                }
                (KeyCode::Char('O'), KeyEventKind::Press) => {
                    data.selection = ui::contest::Selection::OutputExample;
                }

                (KeyCode::Char('j'), KeyEventKind::Press)
                | (KeyCode::Down, KeyEventKind::Press) => match data.selection {
                    ui::contest::Selection::Statement => {
                        data.statements.scroll_state.0 += 1;
                    }
                    ui::contest::Selection::Problems => {
                        data.problems.list_state.select(Some(
                            (data.problems.list_state.selected().unwrap() + 1)
                                % data.problems.vec.unwrap().len(),
                        ));
                    }
                    _ => {}
                },
                (KeyCode::Char('k'), KeyEventKind::Press) | (KeyCode::Up, KeyEventKind::Press) => {
                    match data.selection {
                        ui::contest::Selection::Statement => {
                            if data.statements.scroll_state.0 != 0 {
                                data.statements.scroll_state.0 -= 1;
                            }
                        }
                        ui::contest::Selection::Problems => {
                            data.problems.list_state.select(Some(
                                (data.problems.list_state.selected().unwrap()
                                    + data.problems.vec.unwrap().len()
                                    - 1)
                                    % data.problems.vec.unwrap().len(),
                            ));
                        }
                        _ => {}
                    }
                }
                (KeyCode::Enter, KeyEventKind::Press) => match data.selection {
                    ui::contest::Selection::Problems => {
                        let prob_idx = data.problems.list_state.selected().unwrap();
                        if let None = data.statements.statements[prob_idx] {
                            let (statement, input, output) =
                                backend::get_statement::get_statement_in_out(&urls[prob_idx]);
                            dbg!(input, output);
                            data.statements.statements[prob_idx] = Some(statement);
                            data.statements.scroll_state = (0, 0);
                        }
                        data.statements.now_idx = Some(prob_idx);
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
