use std::error::Error;

use biblatex::ChunksExt;
use clap::{
    crate_authors, crate_description, crate_name, crate_version, ArgMatches,
    Command,
};

use lib::{entry::Entry, shelf::Shelf};

use crate::{
    cli::clap::{add_command, list_command, open_command},
    config::Config,
};

pub struct App {
    config: Config,
    shelf: Shelf,
    cli_commands: Command<'static>,
}

impl App {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let config = Config::get_or_default()?;

        let shelf = Shelf::open(config.db.clone())?;

        let cli_commands = Command::new(crate_name!())
            .about(crate_description!())
            .version(crate_version!())
            .author(crate_authors!())
            .subcommand(add_command())
            .subcommand(open_command())
            .subcommand(list_command());

        Ok(Self {
            config,
            shelf,
            cli_commands,
        })
    }

    pub fn start(self) {
        let matches = self.cli_commands.clone().get_matches();

        match matches.args_present() || matches.subcommand().is_some() {
            true => self.process(&matches), // CLI
            false => todo!(),               // TUI
        }
    }

    fn process(self, matches: &ArgMatches) {
        match matches.subcommand() {
            Some(command) => match command {
                ("add", matches) => self.add_entry(matches),
                ("list", matches) => self.list_entries(matches),
                ("open", matches) => self.open_entry(matches),
                (_, &_) => todo!(),
            },
            None => todo!(),
        }
    }

    fn add_entry(mut self, matches: &ArgMatches) {
        let file_path = matches.value_of_os("FILE").unwrap();

        let mut entry = Entry::new(file_path.to_str().unwrap());

        if let Some(bib_path) = matches.value_of_os("bib") {
            entry = entry.with_bib(bib_path.to_str().unwrap());
        }

        self.shelf.add(entry);

        let file_path = file_path.to_str().unwrap();
        match self.shelf.save(self.config.db) {
            Ok(()) => println!("Successfully added {}", file_path),
            Err(e) => println!("Couldn't add {}: {}", file_path, e),
        }
    }

    fn list_entries(self, matches: &ArgMatches) {
        for entry in self.shelf.entries.iter() {
            let bib_entry = entry.get_bib_entry();
            let path = entry.path.to_str().unwrap();

            if bib_entry.is_some() {
                println!(
                    "{}",
                    bib_entry.unwrap().title().unwrap().format_verbatim()
                );
            } else {
                println!("{}", path);
            }
        }
    }

    fn open_entry(self, matches: &ArgMatches) {
        let entry_path = matches.value_of_os("ENTRY").unwrap();
        let entry = self
            .shelf
            .entries
            .iter()
            .find(|entry| entry.path == entry_path);

        let entry_path = entry_path.to_str().unwrap();

        if entry.is_none() {
            println!("Couldn't open {}: no such entry", entry_path);
        }

        let result = open::that(entry.unwrap().path.clone());

        match result {
            Ok(()) => println!("Successfully opened {}", entry_path),
            Err(e) => println!("Couldn't open {}: {}", entry_path, e),
        }
    }
}
