use wast::parser::{Parse, Parser, Result};

use crate::{Index, TypeUse};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImportDesc {
    Func(ImportDescFunc),
}

impl Parse<'_> for ImportDesc {
    fn parse(parser: Parser<'_>) -> Result<Self> {
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
pub struct ImportDescFunc {
    pub id: Option<Index>,
    pub type_use: TypeUse,
}

impl Parse<'_> for ImportDescFunc {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        parser.parse::<wast::kw::func>()?;

        let id = parser.parse::<Option<Index>>()?;
        let type_use = parser.parse::<TypeUse>()?;

        Ok(Self { id, type_use })
    }
}
