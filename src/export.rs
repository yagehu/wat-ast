use wast::parser::{Cursor, Parse, Parser, Peek, Result};

use crate::FunctionSectionEntry;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Export<'a> {
    pub name: &'a str,
    pub desc: Option<ExportDesc<'a>>,
}

impl<'a> Parse<'a> for Export<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        parser.parse::<wast::kw::export>()?;

        let name = parser.parse::<&str>()?;
        let mut desc = None;

        if !parser.is_empty() {
            desc = Some(parser.parse::<ExportDesc>()?);
        }

        Ok(Self { name, desc })
    }
}

impl Peek for Export<'_> {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.integer().is_some()
    }

    fn display() -> &'static str {
        "integer"
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportDesc<'a> {
    Func(Box<FunctionSectionEntry<'a>>),
}

impl<'a> Parse<'a> for ExportDesc<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let mut l = parser.lookahead1();

        if l.peek::<wast::kw::func>() {
            Ok(Self::Func(Box::new(parser.parse::<FunctionSectionEntry>()?)))
        } else {
            Err(l.error())
        }
    }
}
