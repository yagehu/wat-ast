use std::fmt;

use wast::parser::{Cursor, Parse, Parser, Peek, Result};

use crate::{AsAtoms, Atom};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Integer {
    pub(crate) sign: Option<Sign>,
    pub(crate) src:  String,
    pub(crate) val:  Option<String>,
    pub(crate) hex:  Option<bool>,
}

impl Integer {
    pub fn new(src: String) -> Self {
        Self {
            sign: None,
            src,
            val: None,
            hex: None,
        }
    }

    /// Returns the sign token for this integer.
    pub fn sign(&self) -> Option<Sign> {
        self.sign
    }

    /// Returns the original source text for this integer.
    pub fn src(&self) -> &str {
        &self.src
    }

    /// Returns the value string that can be parsed for this integer, as well as
    /// the base that it should be parsed in
    pub fn val(&self) -> (Option<&String>, Option<u32>) {
        let hex = if let Some(h) = self.hex {
            Some(if h { 16 } else { 10 })
        } else {
            None
        };

        (self.val.as_ref(), hex)
    }
}

impl AsAtoms for Integer {
    fn as_atoms(&self) -> Vec<Atom> {
        vec![Atom::new(self.src.to_owned())]
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.src)
    }
}

impl Parse<'_> for Integer {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        parser.step(|cursor| match cursor.integer() {
            Some((s, cur)) => {
                let src = s.src().to_owned();
                let mut sign = None;
                let (val_ref, base) = s.val();
                let val = Some(val_ref.to_owned());
                let hex = Some(if base == 16 { true } else { false });

                if let Some(si) = s.sign() {
                    match si {
                        wast::lexer::SignToken::Plus => sign = Some(Sign::Pos),
                        wast::lexer::SignToken::Minus => sign = Some(Sign::Neg),
                    }
                }

                Ok((
                    Self {
                        sign,
                        src,
                        val,
                        hex,
                    },
                    cur,
                ))
            },
            None => Err(parser.error("could not parse integer")),
        })
    }
}

impl Peek for Integer {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.integer().is_some()
    }

    fn display() -> &'static str {
        "integer"
    }
}

impl From<i32> for Integer {
    fn from(i: i32) -> Self {
        Self::new(i.to_string())
    }
}

#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Sign {
    Pos,
    Neg,
}
