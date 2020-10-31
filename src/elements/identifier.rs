use std::fmt;

/// https://webassembly.github.io/spec/core/text/values.html#text-id
#[derive(Clone, Debug, PartialEq)]
pub enum Identifier {
    Numeric(u32),
    Symbolic(String),
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Numeric(n) => write!(f, "{}", n),
            Self::Symbolic(s) => write!(f, "${}", s),
        }
    }
}
