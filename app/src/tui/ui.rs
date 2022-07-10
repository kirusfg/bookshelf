use std::io;

use tui::{
    backend::Backend,
    widgets::{Block, Borders},
    Terminal,
};

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
