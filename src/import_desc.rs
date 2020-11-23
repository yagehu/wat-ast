use wast::parser::{Parse, Parser, Result};

use crate::{Atom, Expr, Index, SExpr, TypeUse};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImportDesc {
    Func(ImportDescFunc),
}

impl SExpr for ImportDesc {
    fn car(&self) -> String {
        match self {
            Self::Func(d) => d.car(),
        }
    }

    fn cdr(&self) -> Vec<Expr> {
        match self {
            Self::Func(d) => d.cdr(),
        }
    }
}

impl Parse<'_> for ImportDesc {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        let mut l = parser.lookahead1();

        if l.peek::<wast::kw::func>() {
            Ok(Self::Func(parser.parse::<ImportDescFunc>()?))
        } else {
            Err(l.error())
        }
    }
}

/// https://webassembly.github.io/spec/core/text/modules.html#text-importdesc
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportDescFunc {
    pub idx: Option<Index>,
    pub type_use: TypeUse,
}

impl SExpr for ImportDescFunc {
    fn car(&self) -> String {
        "func".to_owned()
    }

    fn cdr(&self) -> Vec<Expr> {
        let mut v = Vec::new();

        if let Some(ref idx) = self.idx {
            v.push(Expr::Atom(Atom::new(idx.to_string())));
        }

        v.append(&mut self.type_use.exprs());

        v
    }
}

impl Parse<'_> for ImportDescFunc {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        parser.parse::<wast::kw::func>()?;

        let idx = parser.parse::<Option<Index>>()?;
        let type_use = parser.parse::<TypeUse>()?;

        Ok(Self { idx, type_use })
    }
}
