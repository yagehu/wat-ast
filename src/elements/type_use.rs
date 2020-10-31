use super::{Expr, Identifier, Params, Results, SExpr};

#[derive(Clone, Debug, PartialEq)]
pub struct Type(Identifier);

impl SExpr for Type {
    fn car(&self) -> String {
        "type".to_string()
    }

    fn cdr(&self) -> Vec<Expr> {
        vec![Expr::Atom(self.0.to_string())]
    }
}

/// https://webassembly.github.io/spec/core/text/modules.html#text-typeuse
#[derive(Clone, Debug, PartialEq)]
pub struct TypeUse {
    pub type_def: Option<Type>,
    pub params: Vec<Params>,
    pub results: Vec<Results>,
}

impl TypeUse {
    pub fn exprs(&self) -> Vec<Expr> {
        let mut v = Vec::with_capacity(3);

        if let Some(t) = self.type_def.clone() {
            v.push(Expr::SExpr(Box::new(t)));
        }

        for param in self.params.clone() {
            v.push(Expr::SExpr(Box::new(param)));
        }

        for result in self.results.clone() {
            v.push(Expr::SExpr(Box::new(result)));
        }

        v
    }
}
