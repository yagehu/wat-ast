use wast::parser::{self, Parse, Parser};

use crate::{Integer, Param, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueType {
    I32,
    I64,
    F32,
    F64,
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

impl Parse<'_> for MemType {
    fn parse(parser: Parser<'_>) -> parser::Result<Self> {
        let lim = parser.parse::<Limits>()?;

        Ok(Self { lim })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobalType {
    pub is_mut: bool,
    pub val_type: ValueType,
}

impl Parse<'_> for GlobalType {
    fn parse(parser: Parser<'_>) -> parser::Result<Self> {
        let mut is_mut = false;
        let val_type;

        if parser.peek::<wast::LParen>() {
            val_type = parser.parens(|p| {
                p.parse::<wast::kw::r#mut>()?;

                is_mut = true;

                Ok(p.parse::<ValueType>()?)
            })?;
        } else {
            val_type = parser.parse::<ValueType>()?;
        }

        Ok(Self { is_mut, val_type })
    }
}
