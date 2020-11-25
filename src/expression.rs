use wast::parser::{Parse, Parser, Result};

use crate::{
    Atom, Expr, Index, Integer, SExpr, Sign, SymbolicIndex, ValueType,
};

pub fn fold(i: Instruction) -> Expression {
    Expression::Folded(i)
}

pub fn global_get<S: AsRef<str>>(s: S) -> Instruction {
    Instruction::GlobalGet(GlobalGet {
        idx:   Index::Symbolic(SymbolicIndex::new(s.as_ref().to_owned())),
        exprs: vec![],
    })
}

pub fn i32_const<S: AsRef<str>>(
    sign: Option<Sign>,
    s: S,
    hex: bool,
) -> Instruction {
    Instruction::I32Const(I32Const {
        integer: Integer::new(sign, s.as_ref().to_owned(), hex),
        exprs:   vec![],
    })
}

pub fn local_get<S: AsRef<str>>(s: S) -> Instruction {
    Instruction::LocalGet(LocalGet {
        idx:   Index::Symbolic(SymbolicIndex::new(s.as_ref().to_owned())),
        exprs: vec![],
    })
}

enum Paren {
    None,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Level {
    expr:     Expression,
    subexprs: Vec<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Unfolded(Instruction),
    Folded(Instruction),
}

impl Expression {
    pub(crate) fn expr(&self) -> Expr {
        match self {
            Self::Unfolded(i) => Expr::Atom(i.as_atom()),
            Self::Folded(i) => Expr::SExpr(Box::new(i.clone())),
        }
    }

    fn subexprs(&mut self) -> &mut Vec<Expression> {
        match self {
            Self::Unfolded(i) => i.subexprs(),
            Self::Folded(i) => i.subexprs(),
        }
    }
}

#[derive(Default)]
pub(crate) struct ExpressionParser {
    exprs: Vec<Expression>,
    stack: Vec<Level>,
}

impl ExpressionParser {
    pub fn parse(mut self, parser: Parser) -> Result<Vec<Expression>> {
        while !parser.is_empty() || !self.stack.is_empty() {
            match self.paren(parser)? {
                Paren::Left => {
                    let instr = parser.parse::<Instruction>()?;
                    self.stack.push(Level {
                        expr:     Expression::Folded(instr),
                        subexprs: Vec::new(),
                    });
                },
                Paren::None => {
                    let instr = parser.parse::<Instruction>()?;
                    let expr = Expression::Unfolded(instr);

                    match self.stack.last_mut() {
                        Some(level) => level.subexprs.push(expr),
                        None => self.exprs.push(expr),
                    }
                },
                Paren::Right => match self.stack.pop() {
                    Some(mut level) => {
                        level.expr.subexprs().append(&mut level.subexprs);

                        if let Some(top) = self.stack.last_mut() {
                            top.subexprs.push(level.expr);
                        } else {
                            self.exprs.push(level.expr);
                        }
                    },
                    None => {},
                },
            }
        }

        Ok(self.exprs.clone())
    }

    /// Parses either `(`, `)`, or nothing.
    fn paren(&self, parser: Parser) -> Result<Paren> {
        parser.step(|cursor| {
            Ok(match cursor.lparen() {
                Some(rest) => (Paren::Left, rest),
                None if self.stack.is_empty() => (Paren::None, cursor),
                None => match cursor.rparen() {
                    Some(rest) => (Paren::Right, rest),
                    None => (Paren::None, cursor),
                },
            })
        })
    }
}

#[macro_export]
macro_rules! instructions {
    (pub enum Instruction {
        $(
            $name:ident : $keyword:tt : $instr:tt {
                $($field_name:ident: $field_type:ty),*
            },
        )*
    }) => {
        mod kw {
            $(
                wast::custom_keyword!($keyword = $instr);
            )*
        }

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum Instruction {
            $(
                $name($name),
            )*
        }


        impl Instruction {
            pub fn subexprs(&mut self) -> &mut Vec<Expression> {
                match self {
                    $(
                        Self::$name(i) => &mut i.exprs,
                    )*
                }
            }
        }

        impl Instruction {
            pub fn as_atom(&self) -> Atom {
                match self {
                    $(
                        Self::$name(i) => i.as_atom(),
                    )*
                }
            }
        }

        impl Parse<'_> for Instruction {
            fn parse(parser: Parser<'_>) -> Result<Self> {
                let mut l = parser.lookahead1();

                $(
                    if l.peek::<kw::$keyword>() {
                        return Ok(Self::$name(parser.parse()?));
                    }
                )*

                Err(l.error())
            }
        }

        impl SExpr for Instruction {
            fn car(&self) -> std::string::String {
                match self {
                    $(
                        Self::$name(i) => i.car(),
                    )*
                }
            }

            fn cdr(&self) -> Vec<Expr> {
                match self {
                    $(
                        Self::$name(i) => i.cdr(),
                    )*
                }
            }
        }

        $(
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct $name {
                $(
                    pub $field_name: $field_type,
                )*
                pub exprs: Vec<Expression>,
            }

            impl $name {
                pub fn as_atom(&self) -> Atom {
                    #[allow(unused_mut)]
                    let mut s = std::string::String::new();

                    $(
                        s.push(' ');
                        s.push_str(
                            &self
                                .$field_name
                                .as_atoms()
                                .iter()
                                .map(ToString::to_string)
                                .collect::<Vec<std::string::String>>()
                                .join(" ")
                        );
                    )*

                    Atom::new(s)
                }
            }

            impl SExpr for $name {
                fn car(&self) -> std::string::String {
                    format!("{}", $instr)
                }

                fn cdr(&self) -> Vec<Expr> {
                    let mut v = Vec::new();

                    $(
                        v.append(
                            &mut self
                                .clone()
                                .$field_name
                                .as_atoms()
                                .iter()
                                .map(|a| Expr::Atom(a.clone()))
                                .collect()
                        );
                    )*

                    v.append(
                        &mut self
                            .exprs
                            .iter()
                            .map(|e| e.expr())
                            .collect()
                    );

                    v
                }
            }

            impl Parse<'_> for $name {
                fn parse(parser: Parser<'_>) -> Result<Self> {
                    parser.parse::<kw::$keyword>()?;

                    $(
                        let $field_name = parser.parse::<$field_type>()?;
                    )*

                    Ok(Self {
                        $(
                            $field_name,
                        )*
                        exprs: Vec::new(),
                    })
                }
            }
        )*
    };
}

instructions!(
    pub enum Instruction {
        Block     : block      : "block"      { idx: Option<Index> },
        Br        : br         : "br"         { idx: Index },
        BrIf      : br_if      : "br_if"      { idx: Index },
        BrTable   : br_table   : "br_table"   { idxs: Index },
        Call      : call       : "call"       { idx: Index },
        Drop      : drop       : "drop"       {},
        GlobalGet : global_get : "global.get" { idx: Index },
        GlobalSet : global_set : "global.set" { idx: Index },
        I32Add    : i32_add    : "i32.add"    {},
        I32Const  : i32_const  : "i32.const"  { integer: Integer },
        I32Eq     : i32_eq     : "i32.eq"     {},
        I32Eqz    : i32_eqz    : "i32.eqz"    {},
        I32GtU    : i32_gt_u   : "i32.gt_u"   {},
        I32LtU    : i32_lt_u   : "i32.lt_u"   {},
        I32Ne     : i32_ne     : "i32.ne"     {},
        I32RemU   : i32_rem_u  : "i32.rem_u"  {},
        I32Sub    : i32_sub    : "i32.sub"    {},
        I64Const  : i64_const  : "i64.const"  { integer: Integer },
        If        : r#if       : "if"         {},
        Local     : local      : "local"      { idx: Index, value_type: ValueType },
        LocalGet  : local_get  : "local.get"  { idx: Index },
        LocalSet  : local_set  : "local.set"  { idx: Index },
        LocalTee  : local_tee  : "local.tee"  { idx: Index },
        Loop      : r#loop     : "loop"       { idx: Option<Index> },
        Then      : then       : "then"       {},
    }
);

pub trait AsAtoms {
    fn as_atoms(&self) -> Vec<Atom>;
}

impl AsAtoms for String {
    fn as_atoms(&self) -> Vec<Atom> {
        vec![Atom::new(format!(r#""{}""#, self))]
    }
}

impl<T: AsAtoms + Clone> AsAtoms for Option<T> {
    fn as_atoms(&self) -> Vec<Atom> {
        self.clone().map_or(Vec::new(), |x| x.as_atoms())
    }
}
