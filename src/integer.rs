use std::fmt;

use wast::parser::{Cursor, Parse, Parser, Peek, Result};

use crate::ToUnfolded;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Integer {
    sign: Option<Sign>,
    src: String,
    val: String,
    hex: bool,
}

impl Integer {
    pub fn new(
        sign: Option<Sign>,
        src: String,
        val: String,
        hex: bool,
    ) -> Self {
        Self {
            sign,
            src,
            val,
            hex,
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
    pub fn val(&self) -> (&str, u32) {
        (&self.val, if self.hex { 16 } else { 10 })
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.src)
    }
}

impl ToUnfolded for Integer {
    fn to_unfolded(&self) -> String {
        self.to_string()
    }
}

impl Parse<'_> for Integer {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        parser.step(|cursor| match cursor.integer() {
            Some((s, cur)) => {
                let src = s.src().to_owned();
                let mut sign = None;
                let (val_ref, base) = s.val();
                let val = val_ref.to_owned();
                let hex = if base == 16 { true } else { false };

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
            }
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

#[derive(Copy, Debug, Clone, PartialEq, Eq)]
pub enum Sign {
    Pos,
    Neg,
}
