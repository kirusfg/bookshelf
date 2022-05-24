use serde::{Deserialize, Serialize};

/// An entry tag, which can be used for sorting the entries
#[derive(
    Clone,
    Eq,
    Debug,
    Hash,
    PartialEq,
    Deserialize,
    Serialize,
    Ord,
    PartialOrd,
    Default,
)]
pub struct Tag {
    /// The tag itself
    keyword: String,
}

impl Tag {
    /// Creates a new tag from the keyword
    pub fn new(keyword: &str) -> Self {
        Self {
            keyword: keyword.to_string(),
        }
    }
}
