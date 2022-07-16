mod events;
mod state;
mod ui;

use std::io::{self, stdout, Stdout};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, KeyCode},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use tui::{backend::CrosstermBackend, Terminal};

use crate::{app::App, utils::format::format_entry};

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
    pub fn new(app: &'a mut App) -> Self {
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

        let event_loop = EventLoop::default();

        Self {
            app,
            state,
            terminal,
            event_loop,
        }
    }

    /// Runs the TUI life cycle: draw the UI, check for events, gracefully
    /// shutdown on exit.
    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.terminal.draw(|f| ui(f, &mut self.state))?;

        loop {
            let size = self.terminal.size()?;

            self.match_events().await;

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

        shutdown(&mut self.terminal)?;

        Ok(())
    }

    async fn match_events(&mut self) {
        // The TUI is set to redraw at the next frame just for convenience.
        // Be sure to set should_redraw to false whenever you don't need
        // that behavior.
        self.state.should_redraw = true;

        match self.event_loop.rx.recv().await {
            // TODO: think if you need the tick event at all, because the
            // TUI is reactive. Could there be something progress related?
            Some(Event::Tick) => {
                self.state.should_redraw = false;
            },
            Some(Event::Input(key)) => {
                self.match_inputs(key);
            },
            None => self.state.should_exit = true,
        }
    }

    fn match_inputs(&mut self, key: KeyCode) {
        // Prompt interaction
        if self.state.editing_prompt {
            match key {
                KeyCode::Char(c) => {
                    self.state.prompt.push(c);
                },
                KeyCode::Backspace => {
                    self.state.prompt.pop();
                },
                KeyCode::Esc => {
                    self.state.editing_prompt = false;
                },
                KeyCode::Enter => {
                    // Execute the last queued command here
                    todo!();
                },
                _ => {
                    self.state.should_redraw = false;
                },
            }
        } else {
            match key {
                KeyCode::Up | KeyCode::Char('k') => {
                    self.state.entries.previous();
                },
                KeyCode::Down | KeyCode::Char('j') => {
                    self.state.entries.next();
                },
                KeyCode::Enter | KeyCode::Right | KeyCode::Char('l') => {
                    self.open_entry();
                },
                KeyCode::Home | KeyCode::Char('K') => {
                    self.state.entries.first();
                },
                KeyCode::End | KeyCode::Char('J') => {
                    self.state.entries.last();
                },
                KeyCode::Delete | KeyCode::Char('d') => {
                    self.remove_entry();
                },
                KeyCode::Char('a') => {
                    self.state.editing_prompt = true;
                    self.state.prompt.clear();
                },
                KeyCode::Esc | KeyCode::Char('q') => {
                    self.state.should_exit = true;
                },
                _ => {
                    self.state.should_redraw = false;
                },
            }
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

    fn open_entry(&mut self) {
        if let Some(index) = self.state.entries.state.selected() {
            let entry_index = index + 1;
            let entry = self.app.shelf.get_index(entry_index).unwrap();

            self.app.open_entry(entry, None).unwrap();
            self.state.prompt =
                format!("Opened '{}'", format_entry(entry_index, entry));
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

pub(crate) fn shutdown(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), io::Error> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
