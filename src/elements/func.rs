use super::{Export, Expr, Identifier, Instr, SExpr, TypeUse};

/// https://webassembly.github.io/spec/core/text/modules.html#text-func
#[derive(Clone, Debug, PartialEq)]
pub struct Func {
    pub id: Identifier,
    pub type_use: TypeUse,
    pub export: Option<Export>,
    pub instructions: Vec<Instr>,
}

impl SExpr for Func {
    fn car(&self) -> String {
        "func".to_string()
    }

    fn cdr(&self) -> Vec<Expr> {
        let mut v = vec![Expr::Atom(self.id.to_string())];

        v.append(&mut self.type_use.exprs());

        if let Some(export) = self.export.clone() {
            v.push(Expr::SExpr(Box::new(export)));
        }

        v.append(
            &mut self
                .instructions
                .iter()
                .map(|i| Expr::SExpr(Box::new(i.clone())))
                .collect(),
        );

        v
    }
}
