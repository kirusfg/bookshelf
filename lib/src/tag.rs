use serde::{Deserialize, Serialize};

/// An entry tag, which can be used for sorting the entries
#[derive(
    Clone, Eq, PartialEq, Deserialize, Serialize, Ord, PartialOrd, Default,
)]
pub struct Tag {
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
