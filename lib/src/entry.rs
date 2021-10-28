use serde::{Deserialize, Serialize};

use std::{collections::HashSet, fmt::Display, io::Error};

use crate::shelf::Shelf;

#[derive(Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct Author {
    pub name: String,
    pub works: Vec<Entry>,
}

#[derive(Eq, PartialEq, Hash, Deserialize, Serialize)]
pub enum Kind {
    Book,
    Article,
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Kind::Book => write!(f, "a book"),
            Kind::Article => write!(f, "an article"),
        }
    }
}

#[derive(Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct Entry {
    pub title: String,
    pub path: String,
    pub kind: Kind,
    pub author: Option<Vec<Author>>,
    pub year: Option<u32>,
    pub tags: Option<Vec<String>>,
    pub doi: Option<String>,
}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} \"{}\"", self.kind, self.title)
        // TODO: Proper pretty printing for books with multiple authors
    }
}

pub fn add_entry(entry: Entry) -> Result<(), Error> {
    let mut shelf: Shelf = Shelf::read_from_file().unwrap();
    shelf.add_entry(entry);
    shelf.write_to_file()
}

pub fn remove_entry(entry: Entry) -> Result<(), Error> {
    let mut shelf: Shelf = Shelf::read_from_file().unwrap();
    shelf.remove_entry(entry);
    shelf.write_to_file()
}

pub fn get_all_entries() -> Result<HashSet<Entry>, Error> {
    let shelf: Shelf = Shelf::read_from_file().unwrap();
    Ok(shelf.entries)
}
