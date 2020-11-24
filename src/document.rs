use std::fmt;

use wast::parser::{Parse, Parser, Result};

use crate::{Expr, Module, ToWat, ToWatParams};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document {
    module: Module,
}

impl Document {
    pub fn new(module: Module) -> Self {
        Self { module }
    }
}

impl fmt::Display for Document {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            Expr::SExpr(Box::new(self.module.clone())).to_wat(&ToWatParams {
                indent_size:  2,
                indent_level: 0,
            })
        )
    }
}

impl Parse<'_> for Document {
    fn parse(parser: Parser<'_>) -> Result<Self> {
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
                module: Module::with_sections(Vec::new()),
            },
        )
    }

    #[test]
    fn output_empty_module() {
        assert_eq!(
            Document {
                module: Module::with_sections(Vec::new()),
            }
            .to_string(),
            "(module)",
        );
    }
}
