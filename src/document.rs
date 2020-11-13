use wast::parser::{Parse, Parser, Result};

use crate::Module;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document<'a> {
    pub module: Module<'a>,
}

impl<'a> Parse<'a> for Document<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let module = parser.parens(|p| p.parse::<Module>())?;

        Ok(Self { module })
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn empty_module() {
        assert_eq!(
            wast::parser::parse::<Document>(
                &wast::parser::ParseBuffer::new("(module)").unwrap()
            )
            .unwrap(),
            Document {
                module: Module { sections: vec![] },
            },
        )
    }
}
