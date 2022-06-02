use std::fs::{write, File};

use tempfile::{Builder, TempDir};

/// This function provides a made up repository of books, BibTeX files, and
/// similar things useful for testing
pub fn setup() -> TempDir {
    let dir = Builder::new().prefix("my-bookshelf").tempdir().unwrap();

    let files = vec!["book.txt", "another_book.txt", "article.txt"];
    let bib_files = vec!["invalid", "empty.bib", "book.bib"];

    for file in files {
        let _ = File::create(dir.path().join(file));
    }

    for file in bib_files {
        let _ = File::create(dir.path().join(file));
    }

    let bib_entry = "@book{book,
                title     = \"A Good Book\",
                author    = \"Good, Writer\",
                year      = 2022,
                publisher = \"Good Publisher LLC\",
                address   = \"Goodwill\"
            }";
    write(dir.path().join("book.bib"), bib_entry)
        .expect("Failed to write to a file");

    #[cfg(target_family = "windows")]
    std::os::windows::fs::symlink_file(
        dir.path().join("book.txt"),
        dir.path().join("link1.txt"),
    )
    .unwrap();

    #[cfg(target_family = "windows")]
    std::os::windows::fs::symlink_file(
        dir.path().join("book.txt"),
        dir.path().join("link2.txt"),
    )
    .unwrap();

    #[cfg(target_family = "unix")]
    std::os::unix::fs::symlink(
        dir.path().join("book.txt"),
        dir.path().join("link1.txt"),
    )
    .unwrap();

    #[cfg(target_family = "unix")]
    std::os::unix::fs::symlink(
        dir.path().join("book.txt"),
        dir.path().join("link2.txt"),
    )
    .unwrap();

    dir
}
