mod app;
mod cli;
mod config;

use std::error::Error;

use crate::app::App;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new()?;

    app.start();

    Ok(())
}
