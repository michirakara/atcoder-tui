use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, HighlightSpacing, List, ListState, Paragraph},
};

fn get_block(is_activated: bool) -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .border_style(if is_activated {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        })
        .border_type(BorderType::Double)
        .clone()
}

fn gen_list(vec: Vec<&str>) -> List {
    List::new(vec)
        .highlight_symbol(">")
        .highlight_style(Modifier::BOLD)
        .clone()
}

pub fn top_page(f: &mut ratatui::Frame, focus_idx: usize, lis_state: &mut ListState) -> usize {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(5), Constraint::Percentage(95)].as_ref())
        .split(f.size());

    let top_bar = Paragraph::new("AtCoder TUI")
        .block(Block::default().borders(Borders::BOTTOM))
        .alignment(ratatui::layout::Alignment::Left);
    f.render_widget(top_bar, chunks[0]);

    let contests = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(chunks[1]);

    let mut to_ret = 0;

    let active_contests = gen_list(vec!["テスト", "です", "ね〜〜〜〜〜〜〜"])
        .block(get_block(focus_idx == 0).title("Active Contests"));
    if focus_idx == 0 {
        to_ret = active_contests.len();
    }

    let mut tmp = ListState::default();
    f.render_stateful_widget(
        active_contests,
        contests[0],
        if focus_idx == 0 { lis_state } else { &mut tmp },
    );

    let upcoming_contests = gen_list(vec!["テスト", "です", "ね〜〜〜〜〜〜〜"])
        .block(get_block(focus_idx == 1).title("Upcoming Contests"));
    if focus_idx == 1 {
        to_ret = upcoming_contests.len();
    }

    tmp = ListState::default();
    f.render_stateful_widget(
        upcoming_contests,
        contests[1],
        if focus_idx == 1 { lis_state } else { &mut tmp },
    );

    let recent_contests = gen_list(vec!["テスト", "です", "ね〜〜〜〜〜〜〜"])
        .block(get_block(focus_idx == 2).title("Recent Contests"));
    if focus_idx == 2 {
        to_ret = recent_contests.len();
    }

    tmp = ListState::default();
    f.render_stateful_widget(
        recent_contests,
        contests[2],
        if focus_idx == 2 { lis_state } else { &mut tmp },
    );
    return to_ret;
}
