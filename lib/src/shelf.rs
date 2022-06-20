use super::{entry::Entry, tag::Tag};

use bincode::{deserialize, serialize};
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::{Read, Write},
    path::Path,
};

/// A storage for entries, which can be books, articles, etc.
///
/// Can be saved to a file specified as 'db' in the config file.
#[derive(Default, Deserialize, Serialize)]
pub struct Shelf {
    /// Items on a shelf
    pub entries: IndexSet<Entry>,
    /// All tags associated with entries
    pub tags: HashSet<Tag>,
}

impl Shelf {
    /// Adds a new [`Entry`] and its tags to the shelf if those were not
    /// present before. The referenced values are copied. The entries are
    /// stored internally in the order of insertion, and this order is
    /// preserved on removal from the shelf.
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
    pub fn add(&mut self, entry: &Entry) -> bool {
        let is_new = self.entries.insert(entry.clone());

        if is_new && entry.tags.is_some() {
            for tag in entry.tags.as_ref().unwrap().iter() {
                self.tags.insert(tag.clone());
            }
        }

        is_new
    }

    /// Removes the given [`Entry`] from the shelf. Preserves the relative
    /// order of the entries.
    ///
    /// # Returns
    ///
    /// Returns true if the entry was in the shelf before and it was removed,
    /// false otherwise.
    pub fn remove(&mut self, entry: &Entry) -> bool {
        self.entries.shift_remove(entry)
    }

    /// Retrieves an [`Entry`] by its **1-based** index in the set.
    ///
    /// # Returns
    ///
    /// Returns a reference to the [`Entry`] at the (index - 1)th position in
    /// the shelf if the entry is present, and None otherwise.
    pub fn get_index(&self, index: usize) -> Option<&Entry> {
        self.entries.get_index(index - 1)
    }

    /// Removes an [`Entry`] by its **1-based** index in the set.
    /// Preserves the relative order of the entries.
    ///
    /// # Returns
    ///
    /// Returns true if the entry was in the shelf before and it was removed,
    /// false otherwise.
    pub fn remove_index(&mut self, index: usize) -> bool {
        self.entries.shift_remove_index(index - 1).is_some()
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
    pub fn save<P>(&self, file: P) -> Result<(), std::io::Error>
    where
        P: AsRef<Path>,
    {
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
    pub fn open<P>(file: P) -> Result<Shelf, Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let mut binary_data = Vec::new();

        let mut db_file = File::open(file).unwrap();
        db_file.read_to_end(&mut binary_data)?;

        let shelf = deserialize(&binary_data).unwrap();

        Ok(shelf)
    }
}

#[cfg(test)]
mod tests {
    use utils::test::setup;

    use crate::{entry::*, shelf::*, tag::*};

    #[test]
    fn no_duplicate_tags() {
        let dir = setup();

        let entry1 = Entry::new(dir.path().join("book.txt").to_str().unwrap())
            .with_tags(&[Tag::new("fiction")]);
        let entry2 =
            Entry::new(dir.path().join("another_book.txt").to_str().unwrap())
                .with_tags(&[Tag::new("fiction"), Tag::new("classics")]);

        let mut shelf = Shelf::default();

        shelf.add(&entry1);
        shelf.add(&entry2);

        assert_eq!(shelf.tags.len(), 2);

        let mut tags_vec = shelf.tags.iter().collect::<Vec<&Tag>>();
        tags_vec.sort();

        assert_eq!(tags_vec[0], &Tag::new("classics"));
        assert_eq!(tags_vec[1], &Tag::new("fiction"));
    }
}
