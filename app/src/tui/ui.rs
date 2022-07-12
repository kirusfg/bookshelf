use std::ops::Deref;

use tui::{
    backend::Backend,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use super::state::State;

pub(crate) fn ui<B: Backend>(f: &mut Frame<B>, state: &mut State) {
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

    f.render_stateful_widget(list, f.size(), &mut state.entries.state);
}
