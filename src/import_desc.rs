use wast::parser::{Parse, Parser, Result};

use crate::{Identifier, TypeUse};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImportDesc<'a> {
    Func(ImportDescFunc<'a>),
}

impl<'a> Parse<'a> for ImportDesc<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let mut l = parser.lookahead1();

        if l.peek::<wast::kw::func>() {
            Ok(Self::Func(parser.parse::<ImportDescFunc>()?))
        } else {
            Err(l.error())
        }
    }
}

/// https://webassembly.github.io/spec/core/text/modules.html#text-importdesc
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportDescFunc<'a> {
    pub id: Option<Identifier<'a>>,
    pub type_use: TypeUse<'a>,
}

impl<'a> Parse<'a> for ImportDescFunc<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        parser.parse::<wast::kw::func>()?;

        let id = parser.parse::<Option<Identifier>>()?;
        let type_use = parser.parse::<TypeUse>()?;

        Ok(Self { id, type_use })
    }
}
