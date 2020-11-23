use wast::parser::{Parse, Parser, Result};

use crate::{Expr, SExpr, Section};

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

impl SExpr for Module {
    fn car(&self) -> String {
        "module".to_owned()
    }

    fn cdr(&self) -> Vec<Expr> {
        self.sections.iter().flat_map(|s| s.exprs()).collect()
    }
}
