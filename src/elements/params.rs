use super::{Expr, SExpr, ValueType};

/// https://webassembly.github.io/spec/core/text/types.html#text-functype
#[derive(Clone, Debug, PartialEq)]
pub struct Params(Vec<ValueType>);

impl Params {
    pub fn with_value_types(value_types: Vec<ValueType>) -> Self {
        Self(value_types)
    }
}

impl SExpr for Params {
    fn car(&self) -> String {
        "param".to_string()
    }

    fn cdr(&self) -> Vec<Expr> {
        self.0
            .iter()
            .map(|value_type| Expr::Atom(value_type.to_string()))
            .collect()
    }
}
