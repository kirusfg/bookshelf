mod app;
mod cli;
mod config;
mod tui;
mod utils;

use crate::app::App;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new()?;

    app.start().await;

    Ok(())
}
