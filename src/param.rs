use wast::parser::{Parse, Parser, Result};

use crate::{Expr, NamedValueType, SExpr, ValueType};

/// https://webassembly.github.io/spec/core/text/types.html#text-functype
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Param {
    Named(NamedValueType),
    Anonymous(Vec<ValueType>),
}

impl SExpr for Param {
    fn car(&self) -> String {
        "param".to_owned()
    }

    fn cdr(&self) -> Vec<Expr> {
        match self {
            | Self::Named(n) => n.as_exprs(),
            | Self::Anonymous(vv) => {
                vv.iter().map(ValueType::as_expr).collect()
            },
        }
    }
}

impl Parse<'_> for Param {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        if parser.peek::<ValueType>() {
            let mut v = Vec::new();

            while !parser.is_empty() {
                v.push(parser.parse::<ValueType>()?);
            }

            Ok(Self::Anonymous(v))
        } else {
            Ok(Self::Named(parser.parse::<NamedValueType>()?))
        }
    }
}
