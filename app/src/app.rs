use std::error::Error;

use clap::{
    crate_authors, crate_description, crate_name, crate_version, ArgMatches,
    Command,
};

use lib::{entry::Entry, shelf::Shelf};

use crate::{
    cli::clap::{add_command, list_command, open_command, remove_command},
    config::Config,
    utils::format::format_entry,
};

pub struct App {
    /// App configuration struct
    config: Config,
    /// Entry storage
    shelf: Shelf,
    /// Clap commands
    cli_commands: Command<'static>,
}

impl App {
    /// Sets up the `clap` app with information about the program version,
    /// its description and the author, as well as all of the commands.
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let config = Config::get_or_default()?;

        let shelf = Shelf::open(config.db())?;

        let cli_commands = Command::new(crate_name!())
            .about(crate_description!())
            .version(crate_version!())
            .author(crate_authors!())
            .subcommand(add_command())
            .subcommand(remove_command())
            .subcommand(open_command())
            .subcommand(list_command());

        Ok(Self {
            config,
            shelf,
            cli_commands,
        })
    }

    /// Decides whether the user is to run a CLI command or use the TUI
    pub fn start(self) {
        let matches = self.cli_commands.clone().get_matches();

        match matches.args_present() || matches.subcommand().is_some() {
            true => self.run_command(&matches), // CLI
            false => todo!(),                   // TUI
        }
    }

    /// Expands the provided matches and runs the appropriate command
    /// handler.
    fn run_command(self, matches: &ArgMatches) {
        match matches.subcommand() {
            Some(command) => match command {
                ("add", matches) => self.add_entry(matches),
                ("remove", matches) => self.remove_entry(matches),
                ("list", matches) => self.list_entries(matches),
                ("open", matches) => self.open_entry(matches),
                (_, &_) => todo!(),
            },
            None => todo!(),
        }
    }

    /// Extracts a file path from the matches provided, constructs an entry
    /// from it and adds it to the bookshelf.
    fn add_entry(mut self, matches: &ArgMatches) {
        let file_path = matches.value_of_os("FILE").unwrap();

        let mut entry = Entry::new(file_path.to_str().unwrap());

        if let Some(bib_path) = matches.value_of_os("bib") {
            entry = entry.with_bib(bib_path.to_str().unwrap());
        }

        self.shelf.add(&entry);

        let file_path = file_path.to_str().unwrap();
        match self.shelf.save(self.config.db()) {
            Ok(()) => println!("Successfully added {}", file_path),
            Err(e) => println!("Couldn't add {}: {}", file_path, e),
        }
    }

    /// Extracts an entry index from the matches provided, and removes the
    /// corresponding entry from the bookshelf, if it exists.
    fn remove_entry(mut self, matches: &ArgMatches) {
        let entry_index =
            matches.value_of("INDEX").unwrap().parse::<usize>().unwrap();

        let entry = self.shelf.get_index(entry_index);

        if entry.is_none() {
            println!("Couldn't remove entry {}: no such entry", entry_index);
            return;
        }

        let entry = entry.unwrap();
        let entry_name = format_entry(entry_index, entry);

        self.shelf.remove_index(entry_index);

        match self.shelf.save(self.config.db()) {
            Ok(()) => println!("Successfully removed '{}''", entry_name),
            Err(e) => println!("Couldn't remove '{}': {}", entry_name, e),
        }
    }

    /// Extracts an entry index from the matches provided, and opens the
    /// corresponding entry via platform-specific default program, if the
    /// entry exists.
    fn open_entry(self, matches: &ArgMatches) {
        let entry_index =
            matches.value_of("INDEX").unwrap().parse::<usize>().unwrap();

        let entry = self.shelf.get_index(entry_index);

        if entry.is_none() {
            println!("Couldn't open entry {}: no such entry", entry_index);
            return;
        }

        let entry = entry.unwrap();
        let executable = matches
            .value_of_os("exec")
            .map(|os_str| os_str.to_str().unwrap());

        let result = if let Some(executable) = executable {
            open::with(entry.path.clone(), executable)
        } else {
            open::that(entry.path.clone())
        };

        let entry_name = format_entry(entry_index, entry);
        match result {
            Ok(()) if executable.is_some() => println!(
                "Successfully opened '{}' in {}",
                entry_name,
                executable.unwrap()
            ),
            Ok(()) => println!("Successfully opened '{}'", entry_name),
            Err(e) => println!("Couldn't open '{}': {}", entry_name, e),
        }
    }

    /// Simply lists all of the entries on the bookshelf. The output format
    /// depends on whether the entry has a bibliography file associated with
    /// it or not.
    fn list_entries(self, _matches: &ArgMatches) {
        for (i, entry) in self.shelf.entries.iter().enumerate() {
            let entry_name = format_entry(i + 1, entry);
            println!("{}", entry_name);
        }
    }
}
