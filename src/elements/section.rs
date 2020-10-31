use super::{Expr, Func, ImportEntry};

#[derive(Clone, Debug, PartialEq)]
pub enum Section {
    Import(ImportSection),
    Function(FunctionSection),
}

impl Section {
    pub fn exprs(&self) -> Vec<Expr> {
        match self {
            Self::Import(import_section) => import_section.exprs(),
            Self::Function(function_section) => function_section.exprs(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ImportSection(Vec<ImportEntry>);

impl ImportSection {
    pub fn with_entries(entries: Vec<ImportEntry>) -> Self {
        Self(entries)
    }

    pub fn exprs(&self) -> Vec<Expr> {
        self.0
            .iter()
            .map(|e| Expr::SExpr(Box::new(e.clone())))
            .collect()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionSection(Vec<Func>);

impl FunctionSection {
    pub fn with_entries(entries: Vec<Func>) -> Self {
        Self(entries)
    }

    pub fn exprs(&self) -> Vec<Expr> {
        self.0
            .iter()
            .map(|e| Expr::SExpr(Box::new(e.clone())))
            .collect()
    }
}
