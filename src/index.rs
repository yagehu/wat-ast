use wast::parser::{Cursor, Parse, Parser, Peek, Result};

use crate::{Integer, Sign};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Index {
    Numeric(NumericIndex),
    Symbolic(SymbolicIndex),
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
    pub ids: Vec<Index>,
}

impl Parse<'_> for Indexes {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        let mut ids = Vec::new();

        while parser.peek::<Index>() {
            ids.push(parser.parse()?);
        }

        Ok(Self { ids })
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
