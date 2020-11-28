pub use document::Document;
pub use export::{Export, InlineExport};
pub use expression::{
    fold, global_get, i32_const, i64_const, local_get, AsAtoms, Block, Br,
    BrIf, BrTable, Call, Drop, Expression, GlobalGet, GlobalSet, I32Add,
    I32Const, I32Eq, I32Eqz, I32GtU, I32Load, I32LtS, I32LtU, I32Ne, I32RemU,
    I32ShrU, I32Sub, I64Const, If, Instruction, Local, LocalGet, LocalSet,
    LocalTee, Loop, MemoryGrow, Return, Then,
};
pub use import_desc::{ImportDesc, ImportDescFunc};
pub use index::{symbolic, Index, Indexes, NumericIndex, SymbolicIndex};
pub use integer::{Integer, Sign};
pub use module::Module;
pub use named_value_type::NamedValueType;
pub use param::Param;
pub use result::Result;
pub use section::{
    DataSection, DataSectionEntry, DataString, FunctionSection,
    FunctionSectionEntry, GlobalSection, GlobalSectionEntry, ImportSection,
    ImportSectionEntry, MemorySection, MemorySectionEntry, Offset, Section,
    TypeSection, TypeSectionEntry,
};
pub use type_use::TypeUse;
pub use types::{
    FuncType, GlobalType, GlobalTypeMut, Limits, MemType, ValueType,
};

mod document;
mod export;
mod expression;
mod import_desc;
mod index;
mod integer;
mod module;
mod named_value_type;
mod param;
mod result;
mod section;
mod type_use;
mod types;

use std::{fmt, io};

pub(crate) use expression::ExpressionParser;

#[derive(Debug, Clone, PartialEq, Eq)]
struct ToWatParams {
    indent_size:  usize,
    indent_level: usize,
}

impl ToWatParams {
    fn indent(&self) -> usize {
        self.indent_size * self.indent_level
    }
}

trait ToWat {
    fn write_wat<W: io::Write>(
        &self,
        w: &mut W,
        p: &ToWatParams,
    ) -> io::Result<()>;

    fn to_wat(&self, p: &ToWatParams) -> String {
        let mut buf = Vec::new();

        self.write_wat(&mut buf, p).unwrap();

        String::from_utf8_lossy(&buf).to_string()
    }
}

pub enum Expr {
    Atom(Atom),
    SExpr(Box<dyn SExpr>),
}

impl ToWat for Expr {
    fn write_wat<W: io::Write>(
        &self,
        w: &mut W,
        p: &ToWatParams,
    ) -> io::Result<()> {
        match self {
            Self::Atom(a) => {
                write!(w, "{}{}", " ".repeat(p.indent()), a.to_string())
            },
            Self::SExpr(se) => {
                let open = format!("{}({}", " ".repeat(p.indent()), se.car());

                if se.cdr().len() == 0 {
                    return write!(w, "{})", &open);
                }

                let cdr = se
                    .cdr()
                    .iter()
                    .map(|expr| {
                        expr.to_wat(&ToWatParams {
                            indent_size:  2,
                            indent_level: 0,
                        })
                    })
                    .collect::<Vec<String>>()
                    .join(" ");

                if format!("{} {})", open, cdr).len() <= 80 {
                    return write!(w, "{} {})", open, cdr);
                }

                writeln!(w, "{}", open)?;

                for expr in se.cdr() {
                    expr.write_wat(
                        w,
                        &ToWatParams {
                            indent_size:  p.indent_size,
                            indent_level: p.indent_level + 1,
                        },
                    )?;
                    write!(w, "\n")?;
                }

                write!(w, "{})", " ".repeat(p.indent()))
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Atom(String);

impl Atom {
    pub fn new(s: String) -> Atom {
        Self(s)
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub trait SExpr {
    fn car(&self) -> String;

    fn cdr(&self) -> Vec<Expr>;
}
