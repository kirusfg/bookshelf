use super::entry::Entry;

use bincode::{deserialize, serialize};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

/// A storage for entries, which can be books, articles, etc.
///
/// Can be saved to a file specified as 'db' in the config file.
#[derive(Default, Deserialize, Serialize)]
pub struct Shelf {
    /// Items on a shelf
    pub entries: HashSet<Entry>,
}

impl Shelf {
    /// Adds a new [`Entry`] to the shelf.
    ///
    /// # Returns
    ///
    /// Returns true if the entry was not on the shelf before, and false
    /// otherwise.
    ///
    /// # Panics
    ///
    /// Panics if the Entry has an invalid path. The PartialEq implementation
    /// for Entry converts the paths to their canonical form, and if the path
    /// does not exist, program crashes.
    pub fn add(&mut self, entry: Entry) -> bool {
        self.entries.insert(entry)
    }

    /// Removes the given [`Entry`] from the shelf.
    ///
    /// # Returns
    ///
    /// Returns true if the entry was in the shelf before and it was removed,
    /// false otherwise.
    pub fn remove(&mut self, entry: Entry) -> bool {
        self.entries.remove(&entry)
    }

    /// Serializes the [`Shelf`] into a file in binary format.
    ///
    /// # Panics
    ///
    /// Panics if serializing the shelf into binary fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if creating or writing to the file
    /// fails.
    pub fn save(&self, file: PathBuf) -> Result<(), std::io::Error> {
        let binary_data = serialize(&self).unwrap();

        let mut db_file = File::create(file)?;
        db_file.write_all(&binary_data)
    }

    /// Reads a [`Shelf`] from a file in binary format.
    ///
    /// # Panics
    ///
    /// Panics if deserializing the shelf from binary fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if reading from the file fails.
    pub fn open(file: PathBuf) -> Result<Shelf, Box<dyn Error>> {
        let mut binary_data: Vec<u8> = Vec::new();

        let mut db_file = File::open(file).unwrap();
        db_file.read_to_end(&mut binary_data)?;

        let shelf: Shelf = deserialize(&binary_data).unwrap();

        Ok(shelf)
    }
}

#[cfg(test)]
mod tests {}
