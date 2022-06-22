mod app;
mod cli;
mod config;
mod tui;
mod utils;

use std::error::Error;

use crate::app::App;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new()?;

    app.start();

    Ok(())
}
