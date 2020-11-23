use wast::parser::{self, Parse, Parser};

use crate::{Atom, Expr, Index, Param, Result, SExpr};

/// https://webassembly.github.io/spec/core/text/modules.html#text-typeuse
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeUse {
    pub type_def: Option<Type>,
    pub params: Vec<Param>,
    pub results: Vec<Result>,
}

impl TypeUse {
    pub(crate) fn exprs(&self) -> Vec<Expr> {
        let mut v = Vec::new();

        if let Some(ref type_def) = self.type_def {
            v.push(Expr::SExpr(Box::new(type_def.clone())));
        }

        v.append(
            &mut self
                .params
                .iter()
                .map(|p| Expr::SExpr(Box::new(p.clone())))
                .collect(),
        );
        v.append(
            &mut self
                .results
                .iter()
                .map(|r| Expr::SExpr(Box::new(r.clone())))
                .collect(),
        );

        v
    }
}

impl Parse<'_> for TypeUse {
    fn parse(parser: Parser<'_>) -> parser::Result<Self> {
        let mut type_def = None;

        if parser.peek2::<wast::kw::r#type>() {
            type_def = Some(parser.parens(Type::parse)?);
        }

        let mut params = Vec::new();
        let mut results = Vec::new();

        while !parser.is_empty() {
            if parser.peek2::<wast::kw::param>() {
                params.push(parser.parens(Param::parse)?)
            } else {
                break;
            }
        }

        while !parser.is_empty() {
            if parser.peek2::<wast::kw::result>() {
                results.push(parser.parens(Result::parse)?)
            } else {
                break;
            }
        }

        Ok(Self {
            type_def,
            params,
            results,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Type {
    pub idx: Index,
}

impl SExpr for Type {
    fn car(&self) -> String {
        "type".to_owned()
    }

    fn cdr(&self) -> Vec<Expr> {
        vec![Expr::Atom(Atom::new(self.idx.to_string()))]
    }
}

impl Parse<'_> for Type {
    fn parse(parser: Parser<'_>) -> parser::Result<Self> {
        parser.parse::<wast::kw::r#type>()?;

        let idx = parser.parse::<Index>()?;

        Ok(Self { idx })
    }
}
