mod events;
mod state;
mod ui;

use std::{
    io::{self, stdout, Stdout},
    time::Duration,
};

use crossterm::{
    cursor::Show,
    event::{DisableMouseCapture, EnableMouseCapture, KeyCode},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use tui::{backend::CrosstermBackend, Terminal};

use crate::app::App;

use self::{
    events::{Event, EventLoop},
    state::State,
    ui::ui,
};

pub(crate) struct Tui<'a> {
    app: &'a mut App,
    state: State,
    terminal: Terminal<CrosstermBackend<Stdout>>,
    event_loop: EventLoop,
}

impl<'a> Tui<'a> {
    pub fn new(app: &'a mut App, tick_rate: Duration) -> Self {
        let terminal = setup_terminal().unwrap();
        let mut state = State::default();
        state.entries.items = app
            .list_entries()
            .iter()
            .map(|(_, entry)| {
                entry
                    .path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string()
            })
            .collect();

        let event_loop = EventLoop::new(tick_rate);

        Self {
            app,
            state,
            terminal,
            event_loop,
        }
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.terminal.draw(|f| ui(f, &mut self.state))?;

        loop {
            let size = self.terminal.size()?;

            self.match_event().await;

            if self.state.should_exit {
                break;
            }

            let should_redraw =
                self.state.should_redraw || size != self.terminal.size()?;

            if should_redraw {
                self.terminal.draw(|f| ui(f, &mut self.state))?;
                self.state.should_redraw = false;
            }
        }

        shutdown();

        Ok(())
    }

    async fn match_event(&mut self) {
        match self.event_loop.rx.recv().await {
            Some(Event::Tick) => {},
            Some(Event::Input(key)) => match key {
                KeyCode::Up | KeyCode::Char('k') => {
                    self.state.entries.previous();
                    self.state.should_redraw = true;
                },
                KeyCode::Down | KeyCode::Char('j') => {
                    self.state.entries.next();
                    self.state.should_redraw = true;
                },
                KeyCode::Enter | KeyCode::Right | KeyCode::Char('l') => {
                    self.open_entry();
                },
                KeyCode::Delete | KeyCode::Char('d') => {
                    self.remove_entry();
                    self.state.should_redraw = true;
                },
                KeyCode::Home | KeyCode::Char('K') => {
                    self.state.entries.first();
                    self.state.should_redraw = true;
                },
                KeyCode::End | KeyCode::Char('J') => {
                    self.state.entries.last();
                    self.state.should_redraw = true;
                },
                KeyCode::Esc | KeyCode::Char('q') => {
                    self.state.should_exit = true;
                },
                _ => {},
            },
            None => self.state.should_exit = true,
        }
    }

    fn get_entry_list(&mut self) {
        self.state.entries.items = self
            .app
            .list_entries()
            .iter()
            .map(|(_, entry)| {
                entry
                    .path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string()
            })
            .collect();

        if self.state.entries.items.is_empty() {
            self.state.entries.deselect();
        }
    }

    fn open_entry(&self) {
        if let Some(index) = self.state.entries.state.selected() {
            let entry_index = index + 1;
            let entry = self.app.shelf.get_index(entry_index).unwrap();

            self.app.open_entry(entry, None).unwrap();
        }
    }

    fn remove_entry(&mut self) {
        if let Some(index) = self.state.entries.state.selected() {
            let entry_index = index + 1;

            self.app.remove_entry_index(entry_index).unwrap();

            self.get_entry_list();

            // Last item was selected; select a new last item
            if index == self.state.entries.items.len() {
                self.state.entries.last();
            }
        }
    }
}

pub(crate) fn setup_terminal(
) -> Result<Terminal<CrosstermBackend<Stdout>>, io::Error> {
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    Ok(terminal)
}

pub(crate) fn shutdown() {
    // TODO: replace unwraps with try (?)
    disable_raw_mode().unwrap();
    execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture).unwrap();
    execute!(stdout(), Show).unwrap();
}
