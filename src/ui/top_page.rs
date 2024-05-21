use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListState, Paragraph},
};

pub struct ListWithState<'a> {
    pub vec: Option<&'a Vec<String>>,
    pub list_state: ListState,
}
pub struct Data<'a> {
    pub focus_idx: usize,
    pub contests: [ListWithState<'a>; 3],
}

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

fn gen_list(vec: &Vec<String>) -> List<'static> {
    List::new(vec.clone())
        .highlight_symbol(">")
        .highlight_style(Modifier::BOLD)
        .clone()
}

pub fn top_page(f: &mut ratatui::Frame, data: &mut Data) {
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

    let active_contests = gen_list(data.contests[0].vec.unwrap())
        .block(get_block(data.focus_idx == 0).title("Active Contests"));
    f.render_stateful_widget(
        active_contests,
        contests[0],
        &mut data.contests[0].list_state,
    );

    let upcoming_contests = gen_list(data.contests[1].vec.unwrap())
        .block(get_block(data.focus_idx == 1).title("Upcoming Contests"));
    f.render_stateful_widget(
        upcoming_contests,
        contests[1],
        &mut data.contests[1].list_state,
    );

    let recent_contests = gen_list(data.contests[2].vec.unwrap())
        .block(get_block(data.focus_idx == 2).title("Recent Contests"));
    f.render_stateful_widget(
        recent_contests,
        contests[2],
        &mut data.contests[2].list_state,
    );
}
