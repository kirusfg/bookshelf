use serde::{Deserialize, Serialize};

use std::{collections::HashSet, fmt::Display, io::Error};

use crate::shelf::Shelf;

#[derive(Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct Author {
    pub name: String,
    pub works: Vec<Entry>,
}

#[derive(Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
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

#[derive(Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
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

pub fn add_entry(entry: Entry) -> Result<bool, Error> {
    let mut shelf: Shelf = Shelf::read_from_file().unwrap();
    let result: bool = shelf.add_entry(entry);
    shelf.write_to_file()?;
    Ok(result)
}

pub fn remove_entry(entry: Entry) -> Result<bool, Error> {
    let mut shelf: Shelf = Shelf::read_from_file().unwrap();
    let result: bool = shelf.remove_entry(entry);
    shelf.write_to_file()?;
    Ok(result)
}

pub fn get_all_entries() -> Result<HashSet<Entry>, Error> {
    let shelf: Shelf = Shelf::read_from_file().unwrap();
    Ok(shelf.entries)
}

#[cfg(test)]
mod tests {
    use crate::{config::Config, entry::*};

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

        let result: bool = add_entry(entry.clone()).unwrap();
        assert!(result);
        let result: bool = remove_entry(entry.clone()).unwrap();
        assert!(result);

        let entries: HashSet<Entry> = get_all_entries().unwrap();

        assert_eq!(entries.len(), 0);
    }

    #[test]
    fn remove_unexistent_entry() {
        // Test config
        let _config: Config = local_config();

        let entry: Entry = new_entry();
        assert!(!remove_entry(entry).unwrap());
    }
}
