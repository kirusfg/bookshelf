use crate::config::get_config;
use crate::entry::Entry;

use config::Config;

use bincode::{deserialize, serialize};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    fs::File,
    io::{Error, Read, Write},
    path::{Path, PathBuf},
};

#[derive(Default, Deserialize, Serialize)]
pub struct Shelf {
    pub entries: HashSet<Entry>,
}

impl Shelf {
    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.insert(entry);
    }

    pub fn remove_entry(&mut self, entry: Entry) {
        self.entries.remove(&entry);
    }

    pub fn write_to_file(&self) -> Result<(), Error> {
        let config: Config = get_config().unwrap();
        let dirs = ProjectDirs::from("", "", "bookshelf").unwrap();
        let config_path: &Path = dirs.config_dir();

        let file: String = match config.get_str("db") {
            Ok(f) => f,
            Err(_) => String::from("db"),
        };

        let db_file: PathBuf = config_path.join(file);

        let binary_data: Vec<u8> = serialize(&self.entries).unwrap();

        let mut db_file = File::create(db_file).unwrap();
        db_file.write_all(&binary_data)
    }

    pub fn read_from_file() -> Result<Shelf, Error> {
        let config: Config = get_config().unwrap();
        let dirs = ProjectDirs::from("", "", "bookshelf").unwrap();
        let config_path: &Path = dirs.config_dir();

        let file: String = match config.get_str("db") {
            Ok(f) => f,
            Err(_) => String::from("db"),
        };

        let db_file: PathBuf = config_path.join(file);

        let mut binary_data: Vec<u8> = Vec::new();

        let mut db_file = File::open(db_file).unwrap();
        db_file.read_to_end(&mut binary_data)?;

        let shelf: Shelf = deserialize(&binary_data).unwrap();

        Ok(shelf)
    }
}
