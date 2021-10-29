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
    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.insert(entry);
    }

    pub fn remove_entry(&mut self, entry: Entry) {
        self.entries.remove(&entry);
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

        println!("{}", db_file.to_str().unwrap());
        let mut binary_data: Vec<u8> = Vec::new();

        let mut db_file = File::open(db_file).unwrap();
        db_file.read_to_end(&mut binary_data)?;

        let shelf: Shelf = deserialize(&binary_data).unwrap();

        Ok(shelf)
    }
}
