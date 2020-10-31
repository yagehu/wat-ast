use std::fmt;

use super::Identifier;

#[derive(Clone, Debug, PartialEq)]
pub enum Label {
    Id(Identifier),
    Empty,
}

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Id(id) => write!(f, "{}", id.to_string()),
            Self::Empty => write!(f, ""),
        }
    }
}
