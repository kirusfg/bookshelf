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
#[derive(Clone, Default, Debug, Deserialize, Eq, Serialize)]
pub struct Entry {
    /// Path to the entry (can be a link), must be unique
    pub path: PathBuf,
    /// Path to an optional bibliography entry in BibTeX format
    pub bib_path: Option<PathBuf>,
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

    /// Links a BibTeX bibliography to this [`Entry`].
    ///
    /// # Panics
    ///
    /// This function panics if the path to a BibTeX file provided is invalid
    /// (either the file does not exists, or it has a wrong extension).
    pub fn with_bib(mut self, bib_path: &str) -> Self {
        let mut bib_path = PathBuf::from(bib_path);

        match bib_path.canonicalize() {
            Ok(full_path) => bib_path = full_path,
            Err(_) => panic!("The file provided does not exist"),
        }

        // Checking that the file has a .bib extension
        let ext = bib_path.extension();
        if ext.is_none() || ext.unwrap().to_str().unwrap() != "bib" {
            panic!("The file provided is not a BibTeX file");
        }

        self.bib_path = Some(bib_path.clone());

        self
    }

    /// Adds optional tags to this [`Entry`].
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

    /// Returns the BibTeX metadata for this [`Entry`], and None if
    /// the bib_path is None.
    ///
    /// # Panics
    ///
    /// Panics if accessing or parsing the BibTeX file has failed.
    pub fn get_bib_entry(&self) -> Option<BibEntry> {
        // Check if the entry has a BibTeX file associated with it
        self.bib_path.as_ref()?;

        let bib_path = self.bib_path.clone().unwrap();

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

        Some(bib_entry.clone())
    }
}

#[cfg(test)]
mod tests {
    use biblatex::{ChunksExt, EntryType};

    use utils::test::setup;

    use crate::entry::*;

    #[test]
    #[should_panic(expected = "The file provided does not exist")]
    fn file_does_not_exist() {
        let dir = setup();

        let _ =
            Entry::new(dir.path().join("non_existent.pdf").to_str().unwrap());
    }

    #[test]
    fn two_symlinks_same_file() {
        let dir = setup();

        let entry1 = Entry::new(dir.path().join("link1.txt").to_str().unwrap());
        let entry2 = Entry::new(dir.path().join("link2.txt").to_str().unwrap())
            .with_tags(&[Tag::new("fiction")]);

        assert!(entry1 == entry2);
    }

    #[test]
    #[should_panic(expected = "The file provided is not a BibTeX file")]
    fn not_a_bib_file() {
        let dir = setup();

        let _ = Entry::new(dir.path().join("book.txt").to_str().unwrap())
            .with_bib(dir.path().join("invalid").to_str().unwrap());
    }

    #[test]
    #[should_panic(
        expected = "Failed to get the bibliographic entry: invalid cite key"
    )]
    fn empty_bib_file() {
        let dir = setup();

        let book = Entry::new(dir.path().join("book.txt").to_str().unwrap())
            .with_bib(dir.path().join("empty.bib").to_str().unwrap());
        let _bib_entry = book.get_bib_entry().unwrap();
    }

    #[test]
    fn correct_bib_file() {
        let dir = setup();

        let book = Entry::new(dir.path().join("book.txt").to_str().unwrap())
            .with_bib(dir.path().join("book.bib").to_str().unwrap());
        let bib_entry = book.get_bib_entry().unwrap();

        assert_eq!(bib_entry.entry_type, EntryType::Book);
        assert_eq!(bib_entry.title().unwrap().format_verbatim(), "A Good Book");
    }

    #[test]
    #[should_panic]
    fn duplicate_tags() {
        let dir = setup();

        let _ = Entry::new(dir.path().join("book.txt").to_str().unwrap())
            .with_tags(&[Tag::new("fiction"), Tag::new("fiction")]);
    }
}
