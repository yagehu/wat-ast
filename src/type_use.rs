use wast::parser::{self, Parse, Parser};

use crate::{Index, Param, Result};

/// https://webassembly.github.io/spec/core/text/modules.html#text-typeuse
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeUse {
    pub type_def: Option<Type>,
    pub params: Vec<Param>,
    pub results: Vec<Result>,
}

impl Parse<'_> for TypeUse {
    fn parse(parser: Parser<'_>) -> parser::Result<Self> {
        let mut type_def = None;

        if parser.peek2::<wast::kw::r#type>() {
            type_def = Some(parser.parens(Type::parse)?);
        }

        let mut params = Vec::new();
        let mut results = Vec::new();

        while !parser.is_empty() {
            if parser.peek2::<wast::kw::param>() {
                params.push(parser.parens(Param::parse)?)
            } else {
                break;
            }
        }

        while !parser.is_empty() {
            if parser.peek2::<wast::kw::result>() {
                results.push(parser.parens(Result::parse)?)
            } else {
                break;
            }
        }

        Ok(Self {
            type_def,
            params,
            results,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Type {
    pub idx: Index,
}

impl Parse<'_> for Type {
    fn parse(parser: Parser<'_>) -> parser::Result<Self> {
        parser.parse::<wast::kw::r#type>()?;

        let idx = parser.parse::<Index>()?;

        Ok(Self { idx })
    }
}
