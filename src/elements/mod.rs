pub use export::{Export, ExportDesc};
pub use func::Func;
pub use func_type::FuncType;
pub use identifier::Identifier;
pub use import_entry::{ImportDesc, ImportEntry};
pub use instruction::{
    BlockInstr, Call, Else, I32Const, Instr, Loop, PlainInstr, Then,
};
pub use integer::{Decimal, Integer};
pub use label::Label;
pub use module::Module;
pub use name::Name;
pub use params::Params;
pub use results::Results;
pub use section::{FunctionSection, ImportSection, Section};
pub use sign::Sign;
pub use type_use::TypeUse;
pub use value::Value;
pub use value_type::ValueType;

mod export;
mod func;
mod func_type;
mod identifier;
mod import_entry;
mod instruction;
mod integer;
mod label;
mod module;
mod name;
mod params;
mod results;
mod section;
mod sign;
mod type_use;
mod value;
mod value_type;

use std::io;

use super::{ToWat, ToWatParams};

pub enum Expr {
    Atom(String),
    SExpr(Box<dyn SExpr>),
}

impl ToWat for Expr {
    fn write_wat<W: io::Write>(
        &self,
        w: &mut W,
        p: &ToWatParams,
    ) -> io::Result<()> {
        match self {
            Self::Atom(s) => write!(w, "{}{}", " ".repeat(p.indent()), s),
            Self::SExpr(se) => {
                let open = format!("{}({}", " ".repeat(p.indent()), se.car());

                if se.cdr().len() == 0 {
                    return write!(w, "{})", &open);
                }

                let cdr_str = se
                    .cdr()
                    .iter()
                    .map(|expr| {
                        expr.to_wat(&ToWatParams {
                            indent_size: 2,
                            indent_level: 0,
                        })
                    })
                    .collect::<Vec<String>>()
                    .join(" ");

                if format!("{} {})", open, cdr_str).len() <= 80 {
                    return write!(w, "{} {})", open, cdr_str);
                }

                writeln!(w, "{}", open)?;

                for expr in se.cdr() {
                    expr.write_wat(
                        w,
                        &ToWatParams {
                            indent_size: p.indent_size,
                            indent_level: p.indent_level + 1,
                        },
                    )?;
                    write!(w, "\n")?;
                }

                write!(w, "{})", " ".repeat(p.indent()))
            }
        }
    }
}

pub trait SExpr {
    fn car(&self) -> String;

    fn cdr(&self) -> Vec<Expr>;
}
