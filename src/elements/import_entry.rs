use super::{Expr, FuncType, SExpr, Value};

/// https://webassembly.github.io/spec/core/text/modules.html#text-import
#[derive(Clone, Debug, PartialEq)]
pub struct ImportEntry {
    pub module: Value,
    pub name: Value,
    pub desc: ImportDesc,
}

impl SExpr for ImportEntry {
    fn car(&self) -> String {
        "import".to_string()
    }

    fn cdr(&self) -> Vec<Expr> {
        vec![
            Expr::Atom(self.module.to_string()),
            Expr::Atom(self.name.to_string()),
            Expr::SExpr(Box::new(self.desc.clone())),
        ]
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ImportDesc {
    Func(FuncType),
}

impl SExpr for ImportDesc {
    fn car(&self) -> String {
        match self {
            Self::Func(func_type) => func_type.car(),
        }
    }

    fn cdr(&self) -> Vec<Expr> {
        match self {
            Self::Func(func_type) => func_type.cdr(),
        }
    }
}
