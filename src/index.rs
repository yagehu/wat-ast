use wast::parser::{Cursor, Parse, Parser, Peek, Result};

use crate::Integer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Index<'a> {
    Numeric(Integer<'a>),
    Symbolic(SymbolicIndex<'a>),
}

impl<'a> Parse<'a> for Index<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        if let Ok(integer) = parser.parse::<Integer>() {
            Ok(Self::Numeric(integer))
        } else {
            Ok(Self::Symbolic(parser.parse::<SymbolicIndex>()?))
        }
    }
}

impl Peek for Index<'_> {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.id().is_some() || cursor.integer().is_some()
    }

    fn display() -> &'static str {
        "an index"
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Indexes<'a> {
    pub ids: Vec<Index<'a>>,
}

impl<'a> Parse<'a> for Indexes<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let mut ids = Vec::new();

        while parser.peek::<Index>() {
            ids.push(parser.parse()?);
        }

        Ok(Self { ids })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolicIndex<'a>(&'a str);

impl<'a> SymbolicIndex<'a> {
    pub fn name(&self) -> &'a str {
        self.0
    }
}

impl<'a> Parse<'a> for SymbolicIndex<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let id = parser.parse::<wast::Id>()?;

        Ok(Self(id.name()))
    }
}
