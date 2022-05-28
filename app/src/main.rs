mod cli;
mod config;

use clap::{
    arg, crate_authors, crate_description, crate_name, crate_version, Command,
};
use lib::shelf::Shelf;

use crate::{cli::process, config::Config};

fn main() {
    // TODO: move to a separate function
    let matches = Command::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            arg!(-c --config <FILE> "Sets a path to a config file")
                .required(false)
                .allow_invalid_utf8(true),
        )
        .subcommand(
            Command::new("add")
                .about("Adds an entry to your bookshelf")
                .arg(arg!(<FILE>).allow_invalid_utf8(true))
                .arg(
                    arg!(-b --bib <FILE> "Sets a path to the Bib(La)TeX entry")
                        .required(false)
                        .allow_invalid_utf8(true),
                ),
        )
        .get_matches();

    // TODO: move all of the initialization code to a separate function
    let config = match matches.value_of_os("config") {
        Some(_) => Config::default(),
        None => Config::get_or_default().unwrap(),
    };

    let shelf = Shelf::default();

    match matches.args_present() || matches.subcommand().is_some() {
        // CLI
        true => process(matches, config, shelf),
        // TUI
        false => todo!(),
    }
}
