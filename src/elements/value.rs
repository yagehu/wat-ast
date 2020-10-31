use std::fmt;

use super::{Identifier, Integer};

/// https://webassembly.github.io/spec/core/text/values.html
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    /// https://webassembly.github.io/spec/core/text/values.html#integers
    Integer(Integer),

    /// https://webassembly.github.io/spec/core/text/values.html#floating-point
    FloatingPoint(f64),

    /// https://webassembly.github.io/spec/core/text/values.html#strings
    String(String),

    /// https://webassembly.github.io/spec/core/text/values.html#text-id
    Identifier(Identifier),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer(i) => write!(f, "{}", i),
            Self::FloatingPoint(fp) => write!(f, "{}", fp),
            Self::String(s) => write!(f, "\"{}\"", s),
            Self::Identifier(id) => write!(f, "{}", id.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string() {
        assert_eq!(
            Value::String("trusty".to_string()).to_string(),
            "\"trusty\"",
        );
    }
}
