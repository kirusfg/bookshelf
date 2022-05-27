#![deny(missing_docs, clippy::all)]
//! A bookshelf library

/// Entry definition
pub mod entry;

/// Tag definition
pub mod tag;

/// The storage for the entries
pub mod shelf;

#[cfg(test)]
pub mod test_utils {
    use std::{
        env::temp_dir,
        fs::{write, File},
        path::PathBuf,
    };

    /// This function provides a made up repository of books, BibTeX files, and
    /// similar things useful for testing
    pub fn setup() -> PathBuf {
        let dir = temp_dir();

        let _ = File::create(dir.join("book.txt"));
        let _ = File::create(dir.join("another_book.txt"));
        let _ = File::create(dir.join("article.txt"));

        let _ = File::create(dir.join("invalid"));
        let _ = File::create(dir.join("empty.bib"));

        let _ = File::create(dir.join("book.bib"));
        let bib_entry = "@book{book,
                title     = \"A Good Book\",
                author    = \"Good, Writer\",
                year      = 2022,
                publisher = \"Good Publisher LLC\",
                address   = \"Goodwill\"
            }";
        write(dir.join("book.bib"), bib_entry)
            .expect("Failed to write to a file");

        #[cfg(target_family = "windows")]
        let _ = std::os::windows::fs::symlink_file(
            dir.join("book.txt"),
            dir.join("link1.txt"),
        );

        #[cfg(target_family = "windows")]
        let _ = std::os::windows::fs::symlink_file(
            dir.join("book.txt"),
            dir.join("link2.txt"),
        );

        #[cfg(target_family = "unix")]
        let _ = std::os::unix::fs::symlink(
            dir.join("book.txt"),
            dir.join("link1.txt"),
        );

        #[cfg(target_family = "unix")]
        let _ = std::os::unix::fs::symlink(
            dir.join("book.txt"),
            dir.join("link2.txt"),
        );

        dir
    }
}
