use super::{entry::Entry, tag::Tag};

use bincode::{deserialize, serialize};
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    fmt::{Debug, Display},
    fs::File,
    io::{Read, Write},
    path::Path,
};

/// Errors associated with [`Shelf`] operations.
#[derive(Debug)]
pub enum Error {
    /// The entry specified is already on the [`Shelf`].
    DuplicateEntry,
    /// The entry requested is not on the [`Shelf`].
    NoSuchEntry,
    /// Writing the [`Shelf`] to the file specified failed.
    Write,
    /// Writing the [`Shelf`] from the file specified failed.
    Read,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DuplicateEntry => {
                write!(f, "The entry is already on the shelf")
            },
            Error::NoSuchEntry => {
                write!(f, "The requested entry is not on the shelf")
            },
            Error::Write => write!(f, "Writing to the database failed"),
            Error::Read => write!(f, "Writing from the database failed"),
        }
    }
}

impl std::error::Error for Error {}

/// A storage for entries, which can be books, articles, etc., as well as
/// the tags that those entries have.
///
/// Can be saved to/read from a file (in binary format).
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
    /// # Errors
    ///
    /// This function will return an error if the [`Entry`] provided already
    /// existed on the [`Shelf`].
    pub fn add(&mut self, entry: &Entry) -> Result<(), Error> {
        match self.entries.insert(entry.clone()) {
            true => {
                if entry.tags.is_some() {
                    for tag in entry.tags.as_ref().unwrap().iter() {
                        self.tags.insert(tag.clone());
                    }
                }

                Ok(())
            },
            false => Err(Error::DuplicateEntry),
        }
    }

    /// Removes the given [`Entry`] from the shelf. Preserves the relative
    /// order of the entries.
    ///
    /// # Errors
    ///
    /// This function will return an error if there wasn't an [`Entry`] with
    /// the index provided on the [`Shelf`].
    pub fn remove(&mut self, entry: &Entry) -> Result<(), Error> {
        match self.entries.shift_remove(entry) {
            true => Ok(()),
            false => Err(Error::NoSuchEntry),
        }
    }

    /// Retrieves a reference to an [`Entry`] by its **1-based** index.
    ///
    /// # Errors
    ///
    /// This function will return an error if there wasn't an [`Entry`] with
    /// the index provided on the [`Shelf`].
    pub fn get_index(&self, index: usize) -> Result<&Entry, Error> {
        match self.entries.get_index(index - 1) {
            Some(entry) => Ok(entry),
            None => Err(Error::NoSuchEntry),
        }
    }

    /// Removes an [`Entry`] by its **1-based** index on the [`Shelf`].
    /// Preserves the relative order of the entries (insertion order).
    ///
    /// # Errors
    ///
    /// This function will return an error if there wasn't an [`Entry`] with
    /// the index provided on the [`Shelf`].
    pub fn remove_index(&mut self, index: usize) -> Result<(), Error> {
        match self.entries.shift_remove_index(index - 1) {
            Some(_) => Ok(()),
            None => Err(Error::NoSuchEntry),
        }
    }

    /// Serializes the [`Shelf`] into a file in binary format.
    ///
    /// # Errors
    ///
    /// This function will return an error if writing to the file fails,
    /// which can happen when creating the file, serializing the [`Shelf`],
    /// or writing the serialized data to the file.
    pub fn save<P>(&self, file: P) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        let mut db_file = File::create(file).map_err(|_| Error::Write)?;

        let binary_data = serialize(&self).map_err(|_| Error::Write)?;
        db_file.write_all(&binary_data).map_err(|_| Error::Write)
    }

    /// Reads a [`Shelf`] from a file in binary format.
    ///
    /// # Errors
    ///
    /// This function will return an error if reading from the file fails,
    /// which can happen due to file not existing or due to a failure reading
    /// and deserializing its contents.
    pub fn open<P>(file: P) -> Result<Shelf, Error>
    where
        P: AsRef<Path>,
    {
        let mut db_file = File::open(file).map_err(|_| Error::Read)?;

        let mut binary_data = Vec::new();
        db_file
            .read_to_end(&mut binary_data)
            .map_err(|_| Error::Read)?;
        let shelf = deserialize(&binary_data).map_err(|_| Error::Read)?;

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

        assert!(shelf.add(&entry1).is_ok());
        assert!(shelf.add(&entry2).is_ok());

        assert_eq!(shelf.tags.len(), 2);

        let mut tags_vec = shelf.tags.iter().collect::<Vec<&Tag>>();
        tags_vec.sort();

        assert_eq!(tags_vec[0], &Tag::new("classics"));
        assert_eq!(tags_vec[1], &Tag::new("fiction"));
    }
}
