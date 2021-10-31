use lib::config::Config;
use std::io;
use std::time::Duration;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, ListItem};
use tui::Terminal;

fn main() -> Result<(), io::Error> {
    let _config = Config::get_or_default();
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        // Draw all widgets here
        terminal.draw(|f| {
            let size = f.size();
            let _block = Block::default().title("Block").borders(Borders::ALL);
            let books = [
                ListItem::new("Harry Potter"),
                ListItem::new("The Witcher"),
                ListItem::new("The Subtle Art of not Giving a Fuck"),
            ];

            let list = List::new(books)
                .block(Block::default().title("Books").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol(">");

            f.render_widget(list, size);
        })?;

        // TODO Handle key press events
        std::thread::sleep(Duration::from_millis(1000 / 60));
    }
}
