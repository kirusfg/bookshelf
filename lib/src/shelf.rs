use crate::config::Config;
use crate::entry::Entry;

use bincode::{deserialize, serialize};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    fs::File,
    io::{Error, Read, Write},
    path::PathBuf,
};

#[derive(Default, Deserialize, Serialize)]
pub struct Shelf {
    pub entries: HashSet<Entry>,
}

impl Shelf {
    pub fn add_entry(&mut self, entry: Entry) -> bool {
        self.entries.insert(entry)
    }

    pub fn remove_entry(&mut self, entry: Entry) -> bool {
        self.entries.remove(&entry)
    }

    pub fn init() -> Result<(), Error> {
        let config: Config = Config::get_or_default().unwrap();

        let db_file: PathBuf = PathBuf::from(config.db.path);

        let shelf: Shelf = Shelf::default();
        let binary_data: Vec<u8> = serialize(&shelf).unwrap();

        let mut db_file = File::create(db_file).unwrap();
        db_file.write_all(&binary_data)
    }

    pub fn write_to_file(&self) -> Result<(), Error> {
        let config: Config = Config::get_or_default().unwrap();

        let db_file: PathBuf = PathBuf::from(config.db.path);

        let binary_data: Vec<u8> = serialize(&self).unwrap();

        let mut db_file = File::create(db_file).unwrap();
        db_file.write_all(&binary_data)
    }

    pub fn read_from_file() -> Result<Shelf, Error> {
        let config: Config = Config::get_or_default().unwrap();

        let db_file: PathBuf = PathBuf::from(config.db.path);

        if !db_file.exists() {
            Self::init()?;
        }

        let mut binary_data: Vec<u8> = Vec::new();

        let mut db_file = File::open(db_file).unwrap();
        db_file.read_to_end(&mut binary_data)?;

        let shelf: Shelf = deserialize(&binary_data).unwrap();

        Ok(shelf)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        entry::{Entry, Kind},
        shelf::*,
    };

    fn new_entry() -> Entry {
        let entry: Entry = Entry {
            title: String::from("Harry Potter"),
            path: String::from("./harrypotter.pdf"),
            kind: Kind::Book,
            year: Some(2004),
            author: None,
            doi: None,
            tags: None,
        };

        entry
    }

    fn local_config() -> Config {
        let mut config: Config = Config::default();
        config.path = String::from("test.toml");
        config.db.path = String::from("db");

        config
    }

    #[test]
    fn add_and_remove_entry() {
        // Test config
        let _config: Config = local_config();

        let entry: Entry = new_entry();

        let mut shelf: Shelf = Shelf::default();

        assert!(shelf.add_entry(entry.clone()));
        assert!(shelf.remove_entry(entry.clone()));

        assert_eq!(shelf.entries.len(), 0);
    }

    #[test]
    fn remove_unexistent_entry() {
        // Test config
        let _config: Config = local_config();

        let entry: Entry = new_entry();

        let mut shelf: Shelf = Shelf::default();
        assert!(!shelf.remove_entry(entry));
    }

    // TODO Write tests for (de-)serilization of the shelf into file
}
