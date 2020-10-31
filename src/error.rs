use std::{fmt, io, result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    MalformedPattern(String),
    EmptyExpr,
    InvalidChar { idx: usize, string: String },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "i/o error: {}", err),
            Self::MalformedPattern(s) => write!(f, "malformed pattern: {}", s),
            Self::EmptyExpr => write!(f, "expression cannot be empty"),
            Self::InvalidChar { idx, string } => write!(
                f,
                "invalid chararacter at position {} in string: {}",
                idx, string
            ),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}
