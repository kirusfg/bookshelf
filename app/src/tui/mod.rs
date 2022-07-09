pub(crate) mod events;

use std::io::{self, stdout, Stdout};

use crossterm::{
    cursor::Show,
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use tui::{
    backend::{Backend, CrosstermBackend},
    widgets::{Block, Borders},
    Terminal,
};

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
    disable_raw_mode().unwrap();
    execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture).unwrap();
    execute!(stdout(), Show).unwrap();
}

pub(crate) fn draw<B: Backend>(
    terminal: &mut Terminal<B>,
) -> Result<(), io::Error> {
    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default().title("Entries").borders(Borders::ALL);
        f.render_widget(block, size);
    })?;

    Ok(())
}

pub(crate) fn draw_letter<B: Backend>(
    terminal: &mut Terminal<B>,
    letter: &char,
) -> Result<(), io::Error> {
    terminal
        .draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title(letter.to_string())
                .borders(Borders::ALL);
            f.render_widget(block, size);
        })
        .unwrap();

    Ok(())
}
