use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, Borders, List, ListState, Paragraph},
};

pub struct ListWithState<'a> {
    pub vec: Option<&'a Vec<String>>,
    pub list_state: ListState,
}

pub struct StatementWithState<'a> {
    pub statements: Vec<Option<Text<'a>>>,
    pub now_idx: Option<usize>,
    pub scroll_state: (u16, u16),
}

pub enum Selection {
    Problems,
    Submissions,
    Statement,
    InputExample,
    OutputExample,
}

pub struct Data<'a> {
    pub selection: Selection,
    pub problems: ListWithState<'a>,
    pub statements: StatementWithState<'a>,
}

fn gen_problems(problems: &Vec<String>) -> List<'static> {
    List::new(problems.clone())
        .highlight_symbol(">")
        .highlight_style(Modifier::BOLD)
}

fn gen_block(is_activated: bool) -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .border_style(if is_activated {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        })
}

pub fn contest(f: &mut ratatui::Frame, data: &mut Data) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(1), Constraint::Percentage(99)].as_ref())
        .split(f.size());

    let top_bar = Paragraph::new("AtCoder TUI")
        .block(Block::default().borders(Borders::BOTTOM))
        .alignment(ratatui::layout::Alignment::Left);
    f.render_widget(top_bar, chunks[0]);

    let content = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(chunks[1]);

    let left_block = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(content[0]);

    let problems = gen_problems(data.problems.vec.unwrap()).block(
        gen_block(match data.selection {
            Selection::Problems => true,
            _ => false,
        })
        .title("Problems[P]"),
    );
    f.render_stateful_widget(problems, left_block[0], &mut data.problems.list_state);

    let submissions = Paragraph::new("提出の結果")
        .block(
            gen_block(match data.selection {
                Selection::Submissions => true,
                _ => false,
            })
            .title("Result[R]"),
        )
        .alignment(ratatui::layout::Alignment::Left);
    f.render_widget(submissions, left_block[1]);

    let right_block = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(content[1]);

    let statement = Paragraph::new(
        match &data.statements.statements[data.statements.now_idx.unwrap()] {
            None => Text::raw("Problem not loaded"),
            Some(prob) => prob.clone(),
        },
    )
    .block(
        gen_block(match data.selection {
            Selection::Statement => true,
            _ => false,
        })
        .title("Statement[S]"),
    )
    .alignment(ratatui::layout::Alignment::Left)
    .scroll(data.statements.scroll_state);
    f.render_widget(statement, right_block[0]);

    let io_example_div = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(right_block[1]);

    let input_example = Paragraph::new("入力例")
        .block(
            gen_block(match data.selection {
                Selection::InputExample => true,
                _ => false,
            })
            .title("Input Example[I]"),
        )
        .alignment(ratatui::layout::Alignment::Left);
    f.render_widget(input_example, io_example_div[0]);

    let output_example = Paragraph::new("出力例")
        .block(
            gen_block(match data.selection {
                Selection::OutputExample => true,
                _ => false,
            })
            .title("Output Example[O]"),
        )
        .alignment(ratatui::layout::Alignment::Left);
    f.render_widget(output_example, io_example_div[1]);
}
