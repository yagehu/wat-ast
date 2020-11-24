use std::fmt;

use wast::parser::{Cursor, Parse, Parser, Peek, Result};

use crate::{AsAtoms, Atom, Integer, Sign};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Index {
    Numeric(NumericIndex),
    Symbolic(SymbolicIndex),
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Numeric(i) => write!(f, "{}", i.src()),
            Self::Symbolic(i) => write!(f, "{}", i.to_string()),
        }
    }
}

impl Parse<'_> for Index {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        match parser.parse::<NumericIndex>() {
            Ok(ni) => Ok(Self::Numeric(ni)),
            Err(_) => match parser.parse::<SymbolicIndex>() {
                Ok(si) => Ok(Self::Symbolic(si)),
                Err(err) => Err(err),
            },
        }
    }
}

impl AsAtoms for Index {
    fn as_atoms(&self) -> Vec<Atom> {
        match self {
            Self::Numeric(i) => i.as_atoms(),
            Self::Symbolic(i) => i.as_atoms(),
        }
    }
}

impl Peek for Index {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.id().is_some() || cursor.integer().is_some()
    }

    fn display() -> &'static str {
        "an index"
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Indexes {
    pub idxs: Vec<Index>,
}

impl AsAtoms for Indexes {
    fn as_atoms(&self) -> Vec<Atom> {
        self.idxs.iter().map(|i| Atom::new(i.to_string())).collect()
    }
}

impl Parse<'_> for Indexes {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        let mut idxs = Vec::new();

        while parser.peek::<Index>() {
            idxs.push(parser.parse()?);
        }

        Ok(Self { idxs })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NumericIndex {
    i: Integer,
    span: wast::Span,
}

impl NumericIndex {
    pub fn span(&self) -> wast::Span {
        self.span
    }

    /// Returns the sign token for this integer.
    pub fn sign(&self) -> Option<Sign> {
        self.i.sign()
    }

    /// Returns the original source text for this integer.
    pub fn src(&self) -> &str {
        self.i.src()
    }

    /// Returns the value string that can be parsed for this integer, as well as
    /// the base that it should be parsed in
    pub fn val(&self) -> (&str, u32) {
        self.i.val()
    }
}

impl AsAtoms for NumericIndex {
    fn as_atoms(&self) -> Vec<Atom> {
        vec![Atom::new(self.i.to_string())]
    }
}

impl Parse<'_> for NumericIndex {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        let span = parser.cur_span();
        let int = parser.step(|cursor| match cursor.integer() {
            Some((i, cur)) => Ok((i, cur)),
            None => Err(cursor.error("not an integer")),
        })?;
        let sign = int.sign().map(|s| match s {
            wast::lexer::SignToken::Plus => Sign::Pos,
            wast::lexer::SignToken::Minus => Sign::Neg,
        });
        let (val, radix) = int.val();
        let hex = if radix == 16 { true } else { false };
        let i = Integer::new(sign, int.src().to_owned(), val.to_owned(), hex);

        Ok(Self { i, span })
    }
}

impl Peek for NumericIndex {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.integer().is_some()
    }

    fn display() -> &'static str {
        "a numeric index"
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolicIndex {
    name: String,

    /// Span only makes sense when SymbolicIndex was parsed from a token
    /// stream.
    span: Option<wast::Span>,
}

impl SymbolicIndex {
    /// This method can be used when you are building an in-memory data
    /// structure. In that case, there's no need for a span.
    pub fn new(name: String) -> Self {
        Self { name, span: None }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn span(&self) -> Option<wast::Span> {
        self.span
    }
}

impl fmt::Display for SymbolicIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "${}", self.name)
    }
}

impl AsAtoms for SymbolicIndex {
    fn as_atoms(&self) -> Vec<Atom> {
        vec![Atom::new(self.name.clone())]
    }
}

impl Parse<'_> for SymbolicIndex {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        let id = parser.parse::<wast::Id>()?;
        let name = id.name().to_owned();
        let span = Some(id.span());

        Ok(Self { name, span })
    }
}

impl Peek for SymbolicIndex {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.id().is_some()
    }

    fn display() -> &'static str {
        "a symbolic index"
    }
}
