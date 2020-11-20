use wast::parser::{Cursor, Parse, Parser, Peek, Result};

use crate::{Integer, Sign};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Index<'a> {
    Numeric(NumericIndex<'a>),
    Symbolic(SymbolicIndex<'a>),
}

impl<'a> Parse<'a> for Index<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        match parser.parse::<NumericIndex>() {
            Ok(ni) => Ok(Self::Numeric(ni)),
            Err(_) => match parser.parse::<SymbolicIndex>() {
                Ok(si) => Ok(Self::Symbolic(si)),
                Err(err) => Err(err),
            },
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
pub struct NumericIndex<'a> {
    i: Integer<'a>,
    span: wast::Span,
}

impl<'a> NumericIndex<'a> {
    pub fn span(&self) -> wast::Span {
        self.span
    }

    /// Returns the sign token for this integer.
    pub fn sign(&self) -> Option<Sign> {
        self.i.sign()
    }

    /// Returns the original source text for this integer.
    pub fn src(&self) -> &'a str {
        self.i.src()
    }

    /// Returns the value string that can be parsed for this integer, as well as
    /// the base that it should be parsed in
    pub fn val(&self) -> (&str, u32) {
        self.i.val()
    }
}

impl<'a> Parse<'a> for NumericIndex<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
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
        let i = Integer::new(sign, int.src(), val, hex);

        Ok(Self { i, span })
    }
}

impl Peek for NumericIndex<'_> {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.integer().is_some()
    }

    fn display() -> &'static str {
        "a numeric index"
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolicIndex<'a>(wast::Id<'a>);

impl<'a> SymbolicIndex<'a> {
    pub fn name(&self) -> &'a str {
        self.0.name()
    }

    pub fn span(&self) -> wast::Span {
        self.0.span()
    }
}

impl<'a> Parse<'a> for SymbolicIndex<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let id = parser.parse::<wast::Id>()?;

        Ok(Self(id))
    }
}

impl Peek for SymbolicIndex<'_> {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.id().is_some()
    }

    fn display() -> &'static str {
        "a symbolic index"
    }
}
