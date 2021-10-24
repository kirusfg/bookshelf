use lib::entry::{Entry, Kind};

pub fn add_entry(kind: &str, path: &str) {
    let kind: Kind = match kind {
        "book" => Kind::Book,
        "article" => Kind::Article,
        _ => panic!(),
    };

    lib::entry::add_entry(Entry {
        title: String::from(""),
        path: String::from(path),
        kind,
        author: None,
        year: None,
        tags: None,
        doi: None,
    })
}
