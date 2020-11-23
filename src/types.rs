use std::fmt;

use wast::parser::{self, Parse, Parser};

use crate::{Expr, Integer, Param, Result, SExpr, ToUnfolded};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueType {
    I32,
    I64,
    F32,
    F64,
}

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::I32 => write!(f, "i32"),
            Self::I64 => write!(f, "i64"),
            Self::F32 => write!(f, "f32"),
            Self::F64 => write!(f, "f64"),
        }
    }
}

impl ToUnfolded for ValueType {
    fn to_unfolded(&self) -> String {
        self.to_string()
    }
}

impl Parse<'_> for ValueType {
    fn parse(parser: Parser<'_>) -> parser::Result<Self> {
        let mut l = parser.lookahead1();

        if l.peek::<wast::kw::i32>() {
            parser.parse::<wast::kw::i32>()?;
            Ok(Self::I32)
        } else if l.peek::<wast::kw::i64>() {
            parser.parse::<wast::kw::i64>()?;
            Ok(Self::I64)
        } else if l.peek::<wast::kw::f32>() {
            parser.parse::<wast::kw::f32>()?;
            Ok(Self::F32)
        } else if l.peek::<wast::kw::f64>() {
            parser.parse::<wast::kw::f64>()?;
            Ok(Self::F64)
        } else {
            Err(l.error())
        }
    }
}

/// https://webassembly.github.io/spec/core/text/types.html#function-types
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncType {
    pub params: Vec<Param>,
    pub results: Vec<Result>,
}

impl SExpr for FuncType {
    fn car(&self) -> String {
        "func".to_owned()
    }

    fn cdr(&self) -> Vec<Expr> {
        let mut v: Vec<_> = self
            .params
            .iter()
            .map(|p| Expr::SExpr(Box::new(p.clone())))
            .collect();
        let mut results: Vec<_> = self
            .results
            .iter()
            .map(|r| Expr::SExpr(Box::new(r.clone())))
            .collect();

        v.append(&mut results);

        v
    }
}

impl Parse<'_> for FuncType {
    fn parse(parser: Parser<'_>) -> parser::Result<Self> {
        parser.parse::<wast::kw::func>()?;

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

        Ok(Self { params, results })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Limits {
    pub min: Integer,
    pub max: Option<Integer>,
}

impl Limits {
    pub(crate) fn exprs(&self) -> Vec<Expr> {
        let mut v = vec![Expr::Atom(self.min.to_unfolded())];

        if let Some(ref max) = self.max {
            v.push(Expr::Atom(max.to_unfolded()));
        }

        v
    }
}

impl Parse<'_> for Limits {
    fn parse(parser: Parser<'_>) -> parser::Result<Self> {
        let min = parser.parse::<Integer>()?;
        let max = parser.parse::<Option<Integer>>()?;

        Ok(Self { min, max })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemType {
    pub lim: Limits,
}

impl MemType {
    pub(crate) fn exprs(&self) -> Vec<Expr> {
        self.lim.exprs()
    }
}

impl Parse<'_> for MemType {
    fn parse(parser: Parser<'_>) -> parser::Result<Self> {
        let lim = parser.parse::<Limits>()?;

        Ok(Self { lim })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GlobalType {
    Mut(GlobalTypeMut),
    NonMut(ValueType),
}

impl GlobalType {
    pub(crate) fn expr(&self) -> Expr {
        match self {
            Self::Mut(m) => Expr::SExpr(Box::new(m.clone())),
            Self::NonMut(v) => Expr::Atom(v.to_unfolded()),
        }
    }
}

impl Parse<'_> for GlobalType {
    fn parse(parser: Parser<'_>) -> parser::Result<Self> {
        if parser.peek::<wast::LParen>() {
            Ok(Self::Mut(parser.parens(GlobalTypeMut::parse)?))
        } else {
            let val_type = parser.parse::<ValueType>()?;

            Ok(Self::NonMut(val_type))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobalTypeMut {
    pub val_type: ValueType,
}

impl Parse<'_> for GlobalTypeMut {
    fn parse(parser: Parser<'_>) -> parser::Result<Self> {
        parser.parse::<wast::kw::r#mut>()?;

        let val_type = parser.parse::<ValueType>()?;

        Ok(Self { val_type })
    }
}

impl SExpr for GlobalTypeMut {
    fn car(&self) -> String {
        "mut".to_owned()
    }

    fn cdr(&self) -> Vec<Expr> {
        vec![Expr::Atom(self.val_type.to_unfolded())]
    }
}
