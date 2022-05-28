use lib::entry::Entry;
use std::collections::HashSet;

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
    .unwrap();
}

pub fn get_all_entries() {
    let entries: HashSet<Entry> = lib::entry::get_all_entries().unwrap();

    for entry in entries {
        println!("{}", entry);
    }
}
