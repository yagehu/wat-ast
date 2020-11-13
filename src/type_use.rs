use wast::parser::{self, Parse, Parser};

use crate::{Identifier, Param, Result};

/// https://webassembly.github.io/spec/core/text/modules.html#text-typeuse
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeUse<'a> {
    pub type_def: Option<Type<'a>>,
    pub params: Vec<Param>,
    pub results: Vec<Result>,
}

impl<'a> Parse<'a> for TypeUse<'a> {
    fn parse(parser: Parser<'a>) -> parser::Result<Self> {
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
pub struct Type<'a> {
    pub id: Identifier<'a>,
}

impl<'a> Parse<'a> for Type<'a> {
    fn parse(parser: Parser<'a>) -> parser::Result<Self> {
        parser.parse::<wast::kw::r#type>()?;

        let id = parser.parse::<Identifier>()?;

        Ok(Self { id })
    }
}
