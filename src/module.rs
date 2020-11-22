use wast::parser::{Parse, Parser, Result};

use crate::Section;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub sections: Vec<Section>,
}

impl Parse<'_> for Module {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        parser.parse::<wast::kw::module>()?;

        let mut sections = Vec::new();

        while !parser.is_empty() {
            sections.push(parser.parse::<Section>()?)
        }

        Ok(Self { sections })
    }
}
