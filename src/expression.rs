use wast::parser::{Parse, Parser, Result};

use crate::{Expr, Index, Integer, SExpr, ValueType};

enum Paren {
    None,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Level {
    pub expr: Expression,
    pub subexprs: Vec<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Unfolded(Instruction),
    Folded(Instruction),
}

impl Expression {
    pub(crate) fn expr(&self) -> Expr {
        match self {
            Self::Unfolded(i) => Expr::Atom(i.to_unfolded()),
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
pub struct ExpressionParser {
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
                        expr: Expression::Folded(instr),
                        subexprs: Vec::new(),
                    });
                }
                Paren::None => {
                    let instr = parser.parse::<Instruction>()?;
                    let expr = Expression::Unfolded(instr);

                    match self.stack.last_mut() {
                        Some(level) => level.subexprs.push(expr),
                        None => self.exprs.push(expr),
                    }
                }
                Paren::Right => match self.stack.pop() {
                    Some(mut level) => {
                        level.expr.subexprs().append(&mut level.subexprs);

                        if let Some(top) = self.stack.last_mut() {
                            top.subexprs.push(level.expr);
                        } else {
                            self.exprs.push(level.expr);
                        }
                    }
                    None => {}
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

        impl ToUnfolded for Instruction {
            fn to_unfolded(&self) -> String {
                match self {
                    $(
                        Self::$name(i) => i.to_unfolded(),
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
            fn car(&self) -> String {
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

            impl ToUnfolded for $name {
                fn to_unfolded(&self) -> String {
                    #[allow(unused_mut)]
                    let mut s = format!("{}", $instr);

                    $(
                        let argstring = self.$field_name.to_unfolded();

                        if argstring.len() != 0 {
                            s.push(' ');
                            s.push_str(&argstring);
                        }
                    )*

                    s
                }
            }

            impl SExpr for $name {
                fn car(&self) -> String {
                    format!("{}", $instr)
                }

                fn cdr(&self) -> Vec<Expr> {
                    let mut v = vec![
                        $(
                            Expr::Atom(self.$field_name.to_unfolded()),
                        )*
                    ];

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
        Block     : block      : "block"      { id: Option<Index> },
        Br        : br         : "br"         { id: Index },
        BrIf      : br_if      : "br_if"      { id: Index },
        BrTable   : br_table   : "br_table"   { ids: Index },
        Call      : call       : "call"       { id: Index },
        Drop      : drop       : "drop"       {},
        GlobalGet : global_get : "global.get" { id: Index },
        GlobalSet : global_set : "global.set" { id: Index },
        I32Add    : i32_add    : "i32.add"    {},
        I32Const  : i32_const  : "i32.const"  { integer: Integer },
        I32Eq     : i32_eq     : "i32.eq"     {},
        I32Eqz    : i32_eqz    : "i32.eqz"    {},
        I32GtU    : i32_gt_u   : "i32.gt_u"   {},
        I32Ne     : i32_ne     : "i32.ne"     {},
        I32Sub    : i32_sub    : "i32.sub"    {},
        I64Const  : i64_const  : "i64.const"  { integer: Integer },
        If        : r#if       : "if"         {},
        Local     : local      : "local"      { id: Index, value_type: ValueType },
        LocalGet  : local_get  : "local.get"  { id: Index },
        LocalSet  : local_set  : "local.set"  { id: Index },
        LocalTee  : local_tee  : "local.tee"  { id: Index },
        Loop      : r#loop     : "loop"       { id: Option<Index> },
        Then      : then       : "then"       {},
    }
);

pub trait ToUnfolded {
    fn to_unfolded(&self) -> String;
}

impl<T: ToString> ToUnfolded for Option<T> {
    fn to_unfolded(&self) -> String {
        self.as_ref().map_or("".to_owned(), |t| t.to_string())
    }
}
