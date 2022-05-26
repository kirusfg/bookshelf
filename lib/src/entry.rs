use super::tag::Tag;

use biblatex::{Bibliography, Entry as BibEntry};
use serde::{Deserialize, Serialize};
use std::{fs::read_to_string, hash::Hash, path::PathBuf};

/// [`Entry`] is any file that can be contained in your bookshelf.
///
/// Essentially, it is just a path to a file, which in turn can be
/// a book, an article, or a png file of a poster/infographic. This
/// path must be unique to be stored on a shelf - no duplicates
/// are allowed.
#[derive(Clone, Default, Deserialize, Eq, Serialize)]
pub struct Entry {
    /// Path to the entry (can be a link), must be unique
    pub path: PathBuf,
    /// Path to an optional bibliography entry in BibTeX format
    pub bib_path: Option<PathBuf>,
    #[serde(skip)]
    /// Bibliographical entry (from the BibTeX file)
    pub bib_entry: Option<BibEntry>,
    /// Optional list of tags
    pub tags: Option<Vec<Tag>>,
}

impl Hash for Entry {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }

    fn hash_slice<H: std::hash::Hasher>(data: &[Self], state: &mut H)
    where
        Self: Sized,
    {
        for piece in data {
            piece.path.hash(state);
        }
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.path.canonicalize().unwrap() == other.path.canonicalize().unwrap()
    }
}

impl Entry {
    /// Creates a new [`Entry`] form a path string.
    ///
    /// # Panics
    ///
    /// Panics if the file at path provided does not exist
    pub fn new(path: &str) -> Self {
        let mut path = PathBuf::from(path);

        match path.canonicalize() {
            Ok(full_path) => path = full_path,
            Err(_) => panic!("The file provided does not exist"),
        }

        Entry {
            path,
            ..Default::default()
        }
    }

    /// Links a BibTeX bibliography to the entry.
    ///
    /// # Panics
    ///
    /// This function panics if the path to a BibTeX file provided is invalid
    /// (either the file does not exists, or it has a wrong extension).
    pub fn with_bib(mut self, bib_path: PathBuf) -> Self {
        // Checking that the file exists
        if !bib_path.exists() {
            panic!("The file {:?} does not exist!", bib_path);
        }

        // Checking that the file has a .bib extension
        let ext = bib_path.extension();
        if ext.is_none() || ext.unwrap().to_str().unwrap() != "bib" {
            println!("{:?}", ext);
            panic!("The file provided is not a BibTeX file");
        }

        self.bib_path = Some(bib_path.clone());

        // Parsing the file and saving the bibliographical entry
        let bib_str = read_to_string(bib_path.clone()).unwrap_or_else(|e| {
            panic!("Failed to read the contents of the file: {}", e)
        });

        let bibliography = Bibliography::parse(&bib_str).unwrap_or_else(|e| {
            panic!("Failed to parse the bibliographic string: {}", e)
        });

        let cite_key = bib_path
            .file_stem()
            .unwrap_or_else(|| panic!("Failed to extract the cite key"));
        let cite_key = cite_key.to_str().unwrap();

        let bib_entry = bibliography.get(cite_key).unwrap_or_else(|| {
            panic!("Failed to get the bibliographic entry: invalid cite key")
        });

        self.bib_entry = Some(bib_entry.clone());

        self
    }

    /// Adds optional tags to the entry.
    ///
    /// # Panics
    ///
    /// The function panics if there duplicates in the tags array.
    pub fn with_tags(mut self, tags: &[Tag]) -> Self {
        let mut tags_vec = tags.to_vec();
        tags_vec.sort();
        tags_vec.dedup();

        if tags_vec.len() != tags.len() {
            panic!("The tags array contains duplicate tags!");
        }

        self.tags = Some(tags_vec);

        self
    }
}

#[cfg(test)]
mod tests {
    use crate::{entry::*, test_utils::setup};

    #[test]
    #[should_panic(expected = "The file provided does not exist")]
    fn file_does_not_exist() {
        let dir = setup();
        let _ = Entry::new(dir.join("non_existent.pdf").to_str().unwrap());
    }

    #[test]
    fn two_symlinks_same_file() {
        let dir = setup();
        let entry1 = Entry::new(dir.join("link1.txt").to_str().unwrap());
        let entry2 = Entry::new(dir.join("link2.txt").to_str().unwrap())
            .with_tags(&[Tag::new("fiction")]);

        assert!(entry1 == entry2);
    }

    #[test]
    #[should_panic(expected = "The file provided is not a BibTeX file")]
    fn not_a_bib_file() {
        let dir = setup();
        let _ = Entry::new(dir.join("book.txt").to_str().unwrap())
            .with_bib(dir.join("invalid"));
    }

    #[test]
    #[should_panic(
        expected = "Failed to get the bibliographic entry: invalid cite key"
    )]
    fn empty_bib_file() {
        let dir = setup();
        let _ = Entry::new(dir.join("book.txt").to_str().unwrap())
            .with_bib(dir.join("empty.bib"));
    }

    #[test]
    #[should_panic]
    fn duplicate_tags() {
        let dir = setup();
        let _ = Entry::new(dir.join("book.txt").to_str().unwrap())
            .with_tags(&[Tag::new("fiction"), Tag::new("fiction")]);
    }
}
