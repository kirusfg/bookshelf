use tui::widgets::ListState;

#[derive(Clone)]
pub(crate) struct State {
    pub(crate) title: String,
    pub(crate) should_exit: bool,
    pub(crate) should_redraw: bool,
    pub(crate) entries: StatefulList<String>,
    pub(crate) editing_prompt: bool,
    pub(crate) prompt: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            title: String::from("Bookshelf"),
            should_exit: false,
            should_redraw: false,
            entries: StatefulList::default(),
            editing_prompt: false,
            prompt: String::from(""),
        }
    }
}

#[derive(Default, Clone)]
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn deselect(&mut self) {
        self.state.select(None);
    }

    pub fn next(&mut self) {
        let index = match self.state.selected() {
            Some(i) => Some((i + 1) % self.items.len()),
            None if self.items.is_empty() => None,
            None => Some(0),
        };
        self.state.select(index);
    }

    pub fn previous(&mut self) {
        let index = match self.state.selected() {
            Some(i) => Some((i + self.items.len() - 1) % self.items.len()),
            None if self.items.is_empty() => None,
            None => Some(0),
        };
        self.state.select(index);
    }

    pub fn first(&mut self) {
        if !self.items.is_empty() {
            self.state.select(Some(0));
        }
    }
    pub fn last(&mut self) {
        if !self.items.is_empty() {
            self.state.select(Some(self.items.len() - 1));
        }
    }
}
