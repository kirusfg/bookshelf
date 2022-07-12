use tui::widgets::ListState;

#[derive(Clone)]
pub(crate) struct State {
    pub(crate) title: String,
    pub(crate) should_exit: bool,
    pub(crate) should_redraw: bool,
    pub(crate) entries: StatefulList<String>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            title: String::from("Bookshelf"),
            should_exit: false,
            should_redraw: false,
            entries: StatefulList::default(),
        }
    }
}

#[derive(Default, Clone)]
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => (i + 1) % self.items.len(),
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => (i + self.items.len() - 1) % self.items.len(),
            None => 0,
        };
        self.state.select(Some(i));
    }
}
