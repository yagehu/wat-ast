use wast::parser::{Cursor, Parse, Parser, Peek, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Integer<'a> {
    sign: Option<Sign>,
    src: &'a str,
    val: &'a str,
    hex: bool,
}

impl<'a> Integer<'a> {
    /// Returns the sign token for this integer.
    pub fn sign(&self) -> Option<Sign> {
        self.sign
    }

    /// Returns the original source text for this integer.
    pub fn src(&self) -> &'a str {
        self.src
    }

    /// Returns the value string that can be parsed for this integer, as well as
    /// the base that it should be parsed in
    pub fn val(&self) -> (&str, u32) {
        (&self.val, if self.hex { 16 } else { 10 })
    }
}

impl<'a> Parse<'a> for Integer<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        parser.step(|cursor| match cursor.integer() {
            Some((s, cur)) => {
                let src = s.src();
                let mut sign = None;
                let (val, base) = s.val();
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

impl Peek for Integer<'_> {
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
