use wast::parser::{self, Parse, Parser};

use crate::ValueType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Result {
    pub value_types: Vec<ValueType>,
}

impl Parse<'_> for Result {
    fn parse(parser: Parser<'_>) -> parser::Result<Self> {
        parser.parse::<wast::kw::result>()?;

        let mut value_types = Vec::new();

        while !parser.is_empty() {
            value_types.push(parser.parse()?);
        }

        Ok(Self { value_types })
    }
}
