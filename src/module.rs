use wast::parser::{Parse, Parser, Result};

use crate::Section;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module<'a> {
    pub sections: Vec<Section<'a>>,
}

impl<'a> Parse<'a> for Module<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        parser.parse::<wast::kw::module>()?;

        let mut sections = Vec::new();

        while !parser.is_empty() {
            sections.push(parser.parse::<Section>()?)
        }

        Ok(Self { sections })
    }
}
