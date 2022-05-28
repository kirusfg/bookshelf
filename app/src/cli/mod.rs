use std::path::PathBuf;

use clap::ArgMatches;

use lib::{entry::Entry, shelf::Shelf};

use crate::config::Config;

fn add_entry(matches: &ArgMatches, config: Config, mut shelf: Shelf) {
    let file_path = matches.value_of_os("FILE").unwrap();

    let mut entry = Entry::new(file_path.to_str().unwrap());

    if let Some(bib_path) = matches.value_of_os("bib") {
        entry = entry.with_bib(PathBuf::from(bib_path));
    }

    shelf.add(entry);

    let file_path = file_path.to_str().unwrap();
    match shelf.save(config.db) {
        Ok(()) => println!("Successfully added {}", file_path),
        Err(e) => println!("Couldn't add {}: {}", file_path, e),
    }
}

pub fn process(matches: ArgMatches, config: Config, shelf: Shelf) {
    match matches.subcommand() {
        Some(command) => match command {
            ("add", matches) => add_entry(matches, config, shelf),
            (_, &_) => todo!(),
        },
        None => todo!(),
    }
}
