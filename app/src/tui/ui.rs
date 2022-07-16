use std::ops::Deref;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use super::state::State;

pub(crate) fn ui<B: Backend>(f: &mut Frame<B>, state: &mut State) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(f.size());

    let items = state
        .entries
        .items
        .iter()
        .map(|entry| ListItem::new(entry.deref()))
        .collect::<Vec<ListItem>>();
    let list = List::new(items)
        .block(
            Block::default()
                .title(state.title.clone())
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::LightGreen)
                .fg(Color::Black),
        );

    let prompt = Paragraph::new(state.prompt.as_ref())
        .style(match state.editing_prompt {
            false => Style::default(),
            true => Style::default().fg(Color::Yellow),
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(state.prompt_title.clone()),
        );

    f.render_stateful_widget(list, chunks[0], &mut state.entries.state);
    f.render_widget(prompt, chunks[1]);
}
