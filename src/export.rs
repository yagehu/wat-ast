use wast::parser::{Cursor, Parse, Parser, Peek, Result};

use crate::FunctionSectionEntry;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Export {
    pub name: String,
    pub desc: Option<ExportDesc>,
}

impl Parse<'_> for Export {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        parser.parse::<wast::kw::export>()?;

        let name = parser.parse::<String>()?;
        let mut desc = None;

        if !parser.is_empty() {
            desc = Some(parser.parse::<ExportDesc>()?);
        }

        Ok(Self { name, desc })
    }
}

impl Peek for Export {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.integer().is_some()
    }

    fn display() -> &'static str {
        "integer"
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportDesc {
    Func(Box<FunctionSectionEntry>),
}

impl Parse<'_> for ExportDesc {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        let mut l = parser.lookahead1();

        if l.peek::<wast::kw::func>() {
            Ok(Self::Func(Box::new(
                parser.parse::<FunctionSectionEntry>()?,
            )))
        } else {
            Err(l.error())
        }
    }
}
