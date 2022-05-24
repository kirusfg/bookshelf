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
    use std::{env::temp_dir, fs::File, path::PathBuf};

    pub fn setup() -> PathBuf {
        let dir = temp_dir();

        let _ = File::create(dir.join("book.txt"));
        let _ = File::create(dir.join("another_book.txt"));
        let _ = File::create(dir.join("article.txt"));

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
