use wast::parser::{Cursor, Parse, Parser, Peek, Result};

use crate::{Atom, Expr, FunctionSectionEntry, SExpr};

/// https://webassembly.github.io/spec/core/text/modules.html#text-global-abbrev
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InlineExport {
    name: String,
}

impl InlineExport {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl SExpr for InlineExport {
    fn car(&self) -> String {
        "export".to_owned()
    }

    fn cdr(&self) -> Vec<Expr> {
        vec![Expr::Atom(Atom::new(format!(r#""{}""#, self.name)))]
    }
}

impl Parse<'_> for InlineExport {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        parser.parse::<wast::kw::export>()?;

        let name = parser.parse::<String>()?;

        Ok(Self { name })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Export {
    name: String,
    desc: ExportDesc,
}

impl Export {
    pub fn new(name: String, desc: ExportDesc) -> Self {
        Self { name, desc }
    }
}

impl SExpr for Export {
    fn car(&self) -> String {
        "export".to_owned()
    }

    fn cdr(&self) -> Vec<Expr> {
        vec![
            Expr::Atom(Atom::new(format!(r#""{}""#, self.name))),
            Expr::SExpr(Box::new(self.desc.clone())),
        ]
    }
}

impl Parse<'_> for Export {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        parser.parse::<wast::kw::export>()?;

        let name = parser.parse::<String>()?;
        let desc = parser.parse::<ExportDesc>()?;

        Ok(Self { name, desc })
    }
}

impl Peek for Export {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.integer().is_some()
    }

    fn display() -> &'static str {
        "integer"
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportDesc {
    Func(Box<FunctionSectionEntry>),
}

impl SExpr for ExportDesc {
    fn car(&self) -> String {
        match self {
            Self::Func(f) => f.car(),
        }
    }

    fn cdr(&self) -> Vec<Expr> {
        match self {
            Self::Func(f) => f.cdr(),
        }
    }
}

impl Parse<'_> for ExportDesc {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        let mut l = parser.lookahead1();

        if l.peek::<wast::kw::func>() {
            Ok(Self::Func(Box::new(
                parser.parse::<FunctionSectionEntry>()?,
            )))
        } else {
            Err(l.error())
        }
    }
}
