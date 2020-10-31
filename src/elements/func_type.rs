use super::{Expr, Identifier, SExpr, TypeUse};

#[derive(Clone, Debug, PartialEq)]
pub struct FuncType {
    pub id: Identifier,
    pub type_use: TypeUse,
}

impl SExpr for FuncType {
    fn car(&self) -> String {
        "func".to_string()
    }

    fn cdr(&self) -> Vec<Expr> {
        let mut v = vec![Expr::Atom(self.id.to_string())];

        v.append(&mut self.type_use.exprs());

        v
    }
}
