use wast::parser::{Parse, Parser, Result};

use crate::ValueType;

/// https://webassembly.github.io/spec/core/text/types.html#text-functype
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Param {
    pub value_types: Vec<ValueType>,
}

impl Parse<'_> for Param {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        parser.parse::<wast::kw::param>()?;

        let mut value_types = Vec::new();

        while !parser.is_empty() {
            value_types.push(parser.parse()?);
        }

        Ok(Self { value_types })
    }
}
