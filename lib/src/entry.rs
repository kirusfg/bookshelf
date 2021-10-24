use std::fmt::Display;

pub struct Author {
    pub name: String,
    pub works: Vec<Entry>,
}

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

pub fn add_entry(entry: Entry) {
    println!("Added {}", entry)
}
