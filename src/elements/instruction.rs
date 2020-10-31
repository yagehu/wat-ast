use super::{Expr, Identifier, Integer, Label, SExpr};

#[derive(Clone, Debug, PartialEq)]
pub enum Instr {
    PlainInstr(PlainInstr),
    BlockInstr(BlockInstr),
}

impl SExpr for Instr {
    fn car(&self) -> String {
        match self {
            Self::PlainInstr(p) => p.car(),
            Self::BlockInstr(b) => b.car(),
        }
    }

    fn cdr(&self) -> Vec<Expr> {
        match self {
            Self::PlainInstr(p) => p.cdr(),
            Self::BlockInstr(b) => b.cdr(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum BlockInstr {
    Loop(Loop),
}

impl SExpr for BlockInstr {
    fn car(&self) -> String {
        match self {
            Self::Loop(l) => l.car(),
        }
    }

    fn cdr(&self) -> Vec<Expr> {
        match self {
            Self::Loop(l) => l.cdr(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Loop {
    pub label: Label,
    pub then_block: Then,
    pub else_block: Option<Else>,
}

impl SExpr for Loop {
    fn car(&self) -> String {
        "loop".to_string()
    }

    fn cdr(&self) -> Vec<Expr> {
        let mut v = Vec::with_capacity(3);

        if self.label.to_string() != "" {
            v.push(Expr::Atom(self.label.to_string()))
        }

        v.push(Expr::SExpr(Box::new(self.then_block.clone())));

        if let Some(else_block) = self.else_block.clone() {
            v.push(Expr::SExpr(Box::new(else_block)))
        }

        v
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Then(Vec<Instr>);

impl Then {
    pub fn with_instrs(instrs: Vec<Instr>) -> Self {
        Self(instrs)
    }
}

impl SExpr for Then {
    fn car(&self) -> String {
        "then".to_string()
    }

    fn cdr(&self) -> Vec<Expr> {
        self.0
            .iter()
            .map(|instr| Expr::SExpr(Box::new(instr.clone())))
            .collect()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Else(Vec<Instr>);

impl SExpr for Else {
    fn car(&self) -> String {
        "else".to_string()
    }

    fn cdr(&self) -> Vec<Expr> {
        self.0
            .iter()
            .map(|instr| Expr::SExpr(Box::new(instr.clone())))
            .collect()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PlainInstr {
    Call(Call),

    // Numeric Instructions
    // https://webassembly.github.io/spec/core/text/instructions.html#numeric-instructions
    I32Const(I32Const),
}

impl SExpr for PlainInstr {
    fn car(&self) -> String {
        match self {
            Self::Call(c) => c.car(),
            Self::I32Const(i) => i.car(),
        }
    }

    fn cdr(&self) -> Vec<Expr> {
        match self {
            Self::Call(c) => c.cdr(),
            Self::I32Const(i) => i.cdr(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Call {
    pub funcidx: Identifier,
    pub params: Vec<Instr>,
}

impl SExpr for Call {
    fn car(&self) -> String {
        "call".to_string()
    }

    fn cdr(&self) -> Vec<Expr> {
        let mut v = vec![Expr::Atom(self.funcidx.to_string())];

        v.append(
            &mut self
                .params
                .iter()
                .map(|i| Expr::SExpr(Box::new(i.clone())))
                .collect(),
        );

        v
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct I32Const(Integer);

impl I32Const {
    pub fn with_integer(integer: Integer) -> Self {
        Self(integer)
    }
}

impl SExpr for I32Const {
    fn car(&self) -> String {
        "i32.const".to_string()
    }

    fn cdr(&self) -> Vec<Expr> {
        vec![Expr::Atom(self.0.to_string())]
    }
}
