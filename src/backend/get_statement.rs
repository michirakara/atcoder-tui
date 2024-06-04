use ego_tree;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;
use ratatui::text::Text;
use reqwest;
use scraper::{Html, Selector};
use std::ops::Deref;

fn add_lines(to_ret: &mut Text, st: &str, style: Style) {
    let tmp: Vec<&str> = st.split("\n").collect();
    for i in 0..tmp.len() - 1 {
        to_ret.push_span(Span::styled(String::from(tmp[i]), style));
        to_ret.push_line("");
    }
    to_ret.push_span(Span::styled(String::from(tmp[tmp.len() - 1]), style));
}

fn parse(
    elem: ego_tree::NodeRef<scraper::node::Node>,
    mut lis_nest: usize,
    to_ret: &mut Text<'static>,
    input: &mut Vec<Text<'static>>,
    output: &mut Vec<Text<'static>>,
    mut is_input_example: bool,
    mut is_output_example: bool,
) -> (bool, bool) {
    elem.children().for_each(|e| {
        let val = e.value();
        let mut flag = false;
        if let scraper::node::Node::Text(tex) = val {
            if let scraper::node::Node::Element(par_elem) = elem.value().clone() {
                match par_elem.name() {
                    "section" => {
                        let pre_fmt = tex.deref();
                        add_lines(
                            if is_input_example {
                                let tmp = input.len() - 1;
                                &mut input[tmp]
                            } else if is_output_example {
                                let tmp = output.len() - 1;
                                &mut output[tmp]
                            } else {
                                to_ret
                            },
                            &pre_fmt,
                            Style::default(),
                        );
                    }
                    "pre" => {
                        let pre_fmt = tex.deref().replace("\n", "\n| ");
                        add_lines(
                            if is_input_example {
                                let tmp = input.len() - 1;
                                &mut input[tmp]
                            } else if is_output_example {
                                let tmp = output.len() - 1;
                                &mut output[tmp]
                            } else {
                                to_ret
                            },
                            &pre_fmt,
                            Style::default().fg(Color::Blue),
                        );
                    }
                    "h3" => {
                        if let Some(idx) = tex.deref().char_indices().nth(3) {
                            match &tex.deref()[..idx.0] {
                                "入力例" => {
                                    is_output_example = false;
                                    is_input_example = true;
                                    input.push(Text::default());
                                }
                                "出力例" => {
                                    is_input_example = false;
                                    is_output_example = true;
                                    output.push(Text::default());
                                }
                                _ => {}
                            }
                        }
                        let pre_fmt = format!("# {}\n", tex.deref());
                        add_lines(
                            if is_input_example {
                                let tmp = input.len() - 1;
                                &mut input[tmp]
                            } else if is_output_example {
                                let tmp = output.len() - 1;
                                &mut output[tmp]
                            } else {
                                to_ret
                            },
                            &pre_fmt,
                            Style::default()
                                .fg(Color::Blue)
                                .add_modifier(Modifier::BOLD),
                        );
                    }
                    "var" => {
                        let pre_fmt = tex.deref().replace(" _ ", "_");
                        add_lines(
                            if is_input_example {
                                let tmp = input.len() - 1;
                                &mut input[tmp]
                            } else if is_output_example {
                                let tmp = output.len() - 1;
                                &mut output[tmp]
                            } else {
                                to_ret
                            },
                            &pre_fmt,
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::ITALIC),
                        );
                    }
                    "p" => {
                        let pre_fmt = tex.deref();
                        add_lines(
                            if is_input_example {
                                let tmp = input.len() - 1;
                                &mut input[tmp]
                            } else if is_output_example {
                                let tmp = output.len() - 1;
                                &mut output[tmp]
                            } else {
                                to_ret
                            },
                            &pre_fmt,
                            Style::default(),
                        );
                    }
                    "code" => {
                        let pre_fmt = format!("`{}`", tex.deref());
                        add_lines(
                            if is_input_example {
                                let tmp = input.len() - 1;
                                &mut input[tmp]
                            } else if is_output_example {
                                let tmp = output.len() - 1;
                                &mut output[tmp]
                            } else {
                                to_ret
                            },
                            &pre_fmt,
                            Style::default().fg(Color::Red),
                        );
                    }
                    "li" => {
                        let pre_fmt = tex.deref().replace("\n", "");
                        add_lines(
                            if is_input_example {
                                let tmp = input.len() - 1;
                                &mut input[tmp]
                            } else if is_output_example {
                                let tmp = output.len() - 1;
                                &mut output[tmp]
                            } else {
                                to_ret
                            },
                            &pre_fmt,
                            Style::default(),
                        );
                    }
                    "strong" => {
                        let pre_fmt = tex.deref();
                        add_lines(
                            if is_input_example {
                                let tmp = input.len() - 1;
                                &mut input[tmp]
                            } else if is_output_example {
                                let tmp = output.len() - 1;
                                &mut output[tmp]
                            } else {
                                to_ret
                            },
                            &pre_fmt,
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD),
                        );
                    }
                    &_ => {}
                }
            }
        }
        if let scraper::node::Node::Element(now_elem) = val {
            match now_elem.name() {
                "li" => {
                    if is_input_example {
                        let tmp = input.len() - 1;
                        input[tmp].push_line("");
                        input[tmp].push_span(format!("{} - ", "    ".repeat(lis_nest)));
                    } else if is_output_example {
                        let tmp = output.len() - 1;
                        output[tmp].push_line("");
                        output[tmp].push_span(format!("{} - ", "    ".repeat(lis_nest)));
                    } else {
                        to_ret.push_line("");
                        to_ret.push_span(format!("{} - ", "    ".repeat(lis_nest)));
                    }
                    flag = true;
                    lis_nest += 1;
                }
                "pre" => {
                    if is_input_example {
                        let tmp = input.len() - 1;
                        input[tmp].push_span(Span::styled("| ", Style::default().fg(Color::Blue)));
                    } else if is_output_example {
                        let tmp = output.len() - 1;
                        output[tmp].push_span(Span::styled("| ", Style::default().fg(Color::Blue)));
                    } else {
                        to_ret.push_span(Span::styled("| ", Style::default().fg(Color::Blue)));
                    }
                }
                &_ => {}
            }
        }
        if e.has_children() {
            (is_input_example, is_output_example) = parse(
                e,
                lis_nest,
                to_ret,
                input,
                output,
                is_input_example,
                is_output_example,
            );
        }
        if flag {
            lis_nest -= 1;
        }
    });
    (is_input_example, is_output_example)
}

pub fn get_statement_in_out(url: &str) -> (Text<'static>, Vec<Text<'static>>, Vec<Text<'static>>) {
    let res = reqwest::blocking::get(url).unwrap().text().unwrap();
    let selector = Selector::parse(".lang-ja").unwrap();
    let doc = Html::parse_document(&res);
    let elements = doc.select(&selector);
    let paragraphs = elements.clone().next().unwrap().children();

    let mut to_ret = Text::default();
    let mut input: Vec<Text<'static>> = Vec::new();
    let mut output: Vec<Text<'static>> = Vec::new();

    paragraphs.for_each(|e| {
        if e.has_children() {
            parse(e, 0, &mut to_ret, &mut input, &mut output, false, false);
        }
    });
    (to_ret, input, output)
}
