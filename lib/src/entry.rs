use serde::{Deserialize, Serialize};
use std::{hash::Hash, path::PathBuf};

/// [`Entry`] is any file that can be contained in your bookshelf.
///
/// Essentially, it is just a path to a file, which in turn can be
/// a book, an article, or a png file of a poster/infographic. This
/// path must be unique to be stored on a shelf - no duplicates
/// are allowed.
#[derive(Clone, Deserialize, Eq, Serialize)]
pub struct Entry {
    /// Entry id
    pub id: u64,
    /// Path to the entry (can be a link), must be unique
    pub path: PathBuf,
    /// Path to an optional bibliography entry in BibTeX format
    pub bib_path: Option<PathBuf>,
    /// Optional list of tags
    pub tags: Option<Vec<String>>,
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

#[cfg(test)]
mod tests {
    use crate::entry::*;

    // TODO: create two symlinks to the same file programmatically
    fn setup() {
        todo!()
    }

    #[test]
    fn entries_with_same_paths() {
        let entry1 = Entry {
            id: 1,
            path: PathBuf::from("./harry_potter.pdf"),
            bib_path: None,
            tags: Some(vec![String::from("fantasy"), String::from("favorite")]),
        };

        let entry2 = Entry {
            id: 2,
            path: PathBuf::from("./not_harry_potter.pdf"),
            bib_path: None,
            tags: Some(vec![
                String::from("fantasy"),
                String::from("invirogating"),
            ]),
        };

        assert!(entry1 == entry2);
    }
}
