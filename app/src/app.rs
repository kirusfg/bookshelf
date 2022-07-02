use clap::{crate_description, crate_name, crate_version, ArgMatches, Command};

use lib::{entry::Entry, shelf::Shelf};

use crate::{
    cli::{
        add_entry,
        clap::{add_command, list_command, open_command, remove_command},
        list_entries, open_entry, remove_entry,
    },
    config::Config,
    tui::{draw, setup_terminal, shutdown},
};

pub(crate) struct App {
    /// App configuration struct
    pub(crate) config: Config,
    /// Entry storage
    pub(crate) shelf: Shelf,
    /// Clap commands
    pub(crate) cli_commands: Command<'static>,
}

impl App {
    /// Sets up the `clap` app with information about the program version,
    /// its description and the author, as well as all of the commands.
    pub(crate) fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config = Config::get_or_default()?;

        let shelf = Shelf::open(config.db())?;

        let cli_commands = Command::new(crate_name!())
            .about(crate_description!())
            .version(crate_version!())
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

    /// Adds an entry onto the shelf and saves the shelf into a file specified
    /// in the config.
    ///
    /// # Errors
    ///
    /// This function will return an error if the entry already exists, or if
    /// saving the shelf fails.
    pub(crate) fn add_entry(
        &mut self,
        entry: &Entry,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.shelf.add(entry)?;

        Ok(self.shelf.save(self.config.db())?)
    }

    /// Removes the entry with a corresponding index from the bookshelf if
    /// it exists, and saves the shelf into a file specified in the config.
    ///
    /// # Errors
    ///
    /// This function will return an error if the index doesn't correspond to
    /// any existing entries, or if saving the shelf fails.
    pub(crate) fn remove_entry_index(
        &mut self,
        index: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.shelf.remove_index(index)?;

        Ok(self.shelf.save(self.config.db())?)
    }

    /// Opens an entry with a default or specified executable.
    ///
    /// # Errors
    ///
    /// This function will return an error if opening the entry fails.
    pub(crate) fn open_entry(
        &self,
        entry: &Entry,
        exe: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match exe {
            Some(exe) => Ok(open::with(entry.path.clone(), exe)?),
            None => Ok(open::that(entry.path.clone())?),
        }
    }

    /// Simply lists all of the entries on the bookshelf.
    ///
    /// # Returns
    ///
    /// A vector of (index, &[`Entry`]) tuples.
    pub(crate) fn list_entries(&self) -> Vec<(usize, &Entry)> {
        self.shelf.entries.iter().enumerate().collect()
    }

    /// Returns the number of entries on the bookshelf.
    pub(crate) fn size(&self) -> usize {
        self.shelf.entries.len()
    }

    /// Decides whether the user is to run a CLI command or use the TUI
    pub(crate) fn start(&mut self) {
        let matches = self.cli_commands.clone().get_matches();

        match matches.args_present() || matches.subcommand().is_some() {
            true => self.run_command(&matches), // CLI
            false => self.run_tui(),            // TUI
        }
    }

    /// Expands the provided matches and runs the appropriate command handler.
    fn run_command(&mut self, matches: &ArgMatches) {
        match matches.subcommand() {
            Some(command) => match command {
                ("add", matches) => add_entry(self, matches),
                ("remove", matches) => remove_entry(self, matches),
                ("list", matches) => list_entries(self, matches),
                ("open", matches) => open_entry(self, matches),
                (_, &_) => todo!(),
            },
            None => todo!(),
        }
    }

    /// Runs the bookshelf TUI
    fn run_tui(&self) {
        let mut terminal = setup_terminal().unwrap();

        draw(&mut terminal).unwrap();

        shutdown();
    }
}
