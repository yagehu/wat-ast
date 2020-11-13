use wast::parser::{Cursor, Parse, Parser, Peek, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Integer<'a> {
    pub sign: Option<Sign>,
    pub src: &'a str,
}

impl<'a> Parse<'a> for Integer<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        parser.step(|cursor| match cursor.integer() {
            Some((s, cur)) => {
                let src = s.src();
                let mut sign = None;

                if let Some(si) = s.sign() {
                    match si {
                        wast::lexer::SignToken::Plus => sign = Some(Sign::Pos),
                        wast::lexer::SignToken::Minus => sign = Some(Sign::Neg),
                    }
                }

                Ok((Self { sign, src }, cur))
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sign {
    Pos,
    Neg,
}
