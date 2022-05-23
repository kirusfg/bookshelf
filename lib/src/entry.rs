use super::tag::Tag;

use serde::{Deserialize, Serialize};
use std::{hash::Hash, path::PathBuf};

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
            Err(_) => panic!("The file {:?} does not exist!", path),
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
        if !bib_path.exists() {
            panic!("The file {:?} does not exist!", bib_path);
        }

        let ext = bib_path.extension();
        if ext.is_none() || ext.unwrap().to_str().unwrap() != "bib" {
            panic!("The file {:?} is not a BibTeX file!", bib_path);
        }

        self.bib_path = Some(bib_path);

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
    use crate::entry::*;
    use std::{env::temp_dir, fs::File};

    fn setup() -> PathBuf {
        let dir = temp_dir();

        let _ = File::create(dir.join("test.txt"));

        #[cfg(target_family = "windows")]
        let _ = std::os::windows::fs::symlink_file(
            dir.join("test.txt"),
            dir.join("link1.txt"),
        );

        #[cfg(target_family = "windows")]
        let _ = std::os::windows::fs::symlink_file(
            dir.join("test.txt"),
            dir.join("link2.txt"),
        );

        #[cfg(target_family = "unix")]
        let _ = std::os::unix::fs::symlink(
            dir.join("test.txt"),
            dir.join("link1.txt"),
        );

        #[cfg(target_family = "unix")]
        let _ = std::os::unix::fs::symlink(
            dir.join("test.txt"),
            dir.join("link2.txt"),
        );

        dir
    }

    #[test]
    #[should_panic]
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
    #[should_panic]
    fn duplicate_tags() {
        let dir = setup();
        let _ = Entry::new(dir.join("non_existent.pdf").to_str().unwrap())
            .with_tags(&[Tag::new("fiction"), Tag::new("fiction")]);
    }
}
