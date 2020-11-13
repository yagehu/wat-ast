use wast::parser::{Cursor, Parse, Parser, Peek, Result};

use crate::Integer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Identifier<'a> {
    Numeric(Integer<'a>),
    Symbolic(&'a str),
}

impl<'a> Parse<'a> for Identifier<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        if let Ok(integer) = parser.parse::<Integer>() {
            Ok(Self::Numeric(integer))
        } else {
            parser.step(|cursor| match cursor.id() {
                Some((id, cur)) => Ok((Self::Symbolic(id), cur)),
                None => {
                    Err(parser.error(format!("could not parse as identifier")))
                }
            })
        }
    }
}

impl Peek for Identifier<'_> {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.id().is_some() || cursor.integer().is_some()
    }

    fn display() -> &'static str {
        "an identifier"
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifiers<'a> {
    pub ids: Vec<Identifier<'a>>,
}

impl<'a> Parse<'a> for Identifiers<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let mut ids = Vec::new();

        while parser.peek::<Identifier>() {
            ids.push(parser.parse()?);
        }

        Ok(Self { ids })
    }
}
