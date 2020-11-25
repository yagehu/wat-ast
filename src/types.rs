use std::fmt;

use wast::parser::{self, Cursor, Parse, Parser, Peek};

use crate::{AsAtoms, Atom, Expr, Integer, Param, Result, SExpr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueType {
    I32,
    I64,
    F32,
    F64,
}

impl ValueType {
    pub fn as_expr(&self) -> Expr {
        Expr::Atom(Atom::new(self.to_string()))
    }
}

impl AsAtoms for ValueType {
    fn as_atoms(&self) -> Vec<Atom> {
        vec![Atom::new(self.to_string())]
    }
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

impl Peek for ValueType {
    fn peek(cursor: Cursor<'_>) -> bool {
        wast::kw::i32::peek(cursor)
            || wast::kw::i64::peek(cursor)
            || wast::kw::f32::peek(cursor)
            || wast::kw::f64::peek(cursor)
    }

    fn display() -> &'static str {
        "a value type"
    }
}

/// https://webassembly.github.io/spec/core/text/types.html#function-types
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncType {
    params:  Vec<Param>,
    results: Vec<Result>,
}

impl FuncType {
    pub fn new(params: Vec<Param>, results: Vec<Result>) -> Self {
        Self { params, results }
    }
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
    min: Integer,
    max: Option<Integer>,
}

impl Limits {
    pub fn new(min: Integer, max: Option<Integer>) -> Self {
        Self { min, max }
    }

    pub(crate) fn exprs(&self) -> Vec<Expr> {
        let mut v = vec![Expr::Atom(Atom::new(self.min.to_string()))];

        if let Some(ref max) = self.max {
            v.push(Expr::Atom(Atom::new(max.to_string())));
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
    lim: Limits,
}

impl MemType {
    pub fn new(lim: Limits) -> Self {
        Self { lim }
    }

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
            Self::NonMut(v) => Expr::Atom(Atom::new(v.to_string())),
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
    val_type: ValueType,
}

impl GlobalTypeMut {
    pub fn new(val_type: ValueType) -> Self {
        Self { val_type }
    }
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
        vec![Expr::Atom(Atom::new(self.val_type.to_string()))]
    }
}
