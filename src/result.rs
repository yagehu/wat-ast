use wast::parser::{self, Parse, Parser};

use crate::{Atom, Expr, SExpr, ValueType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Result {
    pub value_types: Vec<ValueType>,
}

impl SExpr for Result {
    fn car(&self) -> String {
        "result".to_owned()
    }

    fn cdr(&self) -> Vec<Expr> {
        self.value_types
            .iter()
            .map(|v| Expr::Atom(Atom::new(v.to_string())))
            .collect()
    }
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
