use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Sign {
    Empty,
    Positive,
    Negative,
}

impl fmt::Display for Sign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, ""),
            Self::Positive => write!(f, "+"),
            Self::Negative => write!(f, "-"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(Sign::Empty.to_string(), "");
    }

    #[test]
    fn positive() {
        assert_eq!(Sign::Positive.to_string(), "+");
    }

    #[test]
    fn negative() {
        assert_eq!(Sign::Negative.to_string(), "-");
    }
}
