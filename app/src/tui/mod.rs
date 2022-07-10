pub(crate) mod events;
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

use self::{
    events::{Event, EventLoop},
    ui::{draw, draw_letter},
};

pub(crate) struct Tui {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    event_loop: EventLoop,
}

impl Tui {
    pub fn new(tick_rate: Duration) -> Self {
        let terminal = setup_terminal().unwrap();

        let event_loop = EventLoop::new(tick_rate);

        Self {
            terminal,
            event_loop,
        }
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        draw(&mut self.terminal)?;

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
                    KeyCode::Char(c) => draw_letter(&mut self.terminal, &c)?,
                    KeyCode::Null => todo!(),
                    KeyCode::Esc => break,
                },
                None => break,
            }

            if size != self.terminal.size()? {
                draw(&mut self.terminal)?;
            }
        }

        shutdown();

        Ok(())
    }
}

impl Default for Tui {
    fn default() -> Self {
        Self::new(Duration::from_millis(1000 / 120)) // 120 FPS
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
