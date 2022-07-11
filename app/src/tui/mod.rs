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
use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders},
    Terminal,
};

use crate::app::App;

use self::{
    events::{Event, EventLoop},
    state::State,
    ui::ui,
};

pub(crate) struct Tui<'a> {
    app: &'a App,
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

            match self.event_loop.rx.recv().await {
                Some(Event::Tick) => {},
                Some(Event::Input(key)) => match key {
                    KeyCode::Backspace => todo!(),
                    KeyCode::Enter => todo!(),
                    KeyCode::Left => todo!(),
                    KeyCode::Right => todo!(),
                    KeyCode::Up => todo!(),
                    KeyCode::Down => todo!(),
                    KeyCode::Home => todo!(),
                    KeyCode::End => todo!(),
                    KeyCode::PageUp => todo!(),
                    KeyCode::PageDown => todo!(),
                    KeyCode::Tab => todo!(),
                    KeyCode::BackTab => todo!(),
                    KeyCode::Delete => todo!(),
                    KeyCode::Insert => todo!(),
                    KeyCode::F(_) => todo!(),
                    KeyCode::Char(c) => match c {
                        'q' => self.state.should_exit = true,
                        c => {
                            self.state.title = c.to_string();
                            self.state.should_redraw = true;
                        },
                    },
                    KeyCode::Null => todo!(),
                    KeyCode::Esc => self.state.should_exit = true,
                },
                None => self.state.should_exit = true,
            }

            if self.state.should_exit {
                break;
            }

            let should_redraw =
                self.state.should_redraw || size != self.terminal.size()?;

            if should_redraw {
                self.terminal.draw(|f| ui(f, &mut self.state))?;
            }
        }

        shutdown();

        Ok(())
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
