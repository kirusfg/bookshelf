use std::ops::Deref;

use tui::{
    backend::Backend,
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use super::state::State;

pub(crate) fn ui<B: Backend>(f: &mut Frame<B>, state: &mut State) {
    let size = f.size();
    let block = Block::default()
        .title(state.title.clone())
        .borders(Borders::ALL);

    let items = state
        .entries
        .items
        .iter()
        .map(|entry| ListItem::new(entry.deref()))
        .collect::<Vec<ListItem>>();
    let list = List::new(items);
    let list_size = block.inner(size);

    f.render_widget(block, size);
    f.render_stateful_widget(list, list_size, &mut state.entries.state);
}
