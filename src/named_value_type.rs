use wast::parser::{Parse, Parser, Result};

use crate::{AsAtoms, Atom, Expr, SymbolicIndex, ValueType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedValueType {
    idx:        SymbolicIndex,
    value_type: ValueType,
}

impl NamedValueType {
    pub fn new(idx: SymbolicIndex, value_type: ValueType) -> Self {
        Self { idx, value_type }
    }

    pub fn as_exprs(&self) -> Vec<Expr> {
        self.as_atoms().into_iter().map(|a| Expr::Atom(a)).collect()
    }
}

impl AsAtoms for NamedValueType {
    fn as_atoms(&self) -> Vec<Atom> {
        vec![
            Atom::new(self.idx.to_string()),
            Atom::new(self.value_type.to_string()),
        ]
    }
}

impl Parse<'_> for NamedValueType {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        let idx = parser.parse::<SymbolicIndex>()?;
        let value_type = parser.parse::<ValueType>()?;

        Ok(Self { idx, value_type })
    }
}
