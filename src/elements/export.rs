use super::{Expr, Func, Name, SExpr};

#[derive(Clone, Debug, PartialEq)]
pub struct Export {
    pub name: Name,
    pub desc: Option<Box<ExportDesc>>,
}

impl SExpr for Export {
    fn car(&self) -> String {
        "export".to_string()
    }

    fn cdr(&self) -> Vec<Expr> {
        let mut v = vec![
            Expr::Atom(self.name.to_string()),
        ];

        if let Some(desc) = self.desc.clone() {
            v.push(Expr::SExpr(desc));
        }

        v
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExportDesc {
    Func(Func),
}

impl SExpr for ExportDesc {
    fn car(&self) -> String {
        match self {
            Self::Func(func) => func.car(),
        }
    }

    fn cdr(&self) -> Vec<Expr> {
        match self {
            Self::Func(func) => func.cdr(),
        }
    }
}
