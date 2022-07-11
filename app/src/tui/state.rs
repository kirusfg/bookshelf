use tui::widgets::ListState;

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

#[derive(Default)]
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            },
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            },
            None => 0,
        };
        self.state.select(Some(i));
    }
}
