use super::{Expr, SExpr, ValueType};

#[derive(Clone, Debug, PartialEq)]
pub struct Results(Vec<ValueType>);

impl SExpr for Results {
    fn car(&self) -> String {
        "result".to_string()
    }

    fn cdr(&self) -> Vec<Expr> {
        self.0
            .iter()
            .map(|value_type| Expr::Atom(value_type.to_string()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ToWat, ToWatParams};

    #[test]
    fn empty() {
        assert_eq!(
            Expr::SExpr(Box::new(Results(Vec::new())))
                .to_wat(&ToWatParams::default()),
            "(result)"
        );
    }

    #[test]
    fn one_value_type() {
        assert_eq!(
            Expr::SExpr(Box::new(Results(vec![ValueType::I32])))
                .to_wat(&ToWatParams::default()),
            "(result i32)",
        );
    }

    #[test]
    fn multiple_value_types() {
        assert_eq!(
            Expr::SExpr(Box::new(Results(vec![
                ValueType::I32,
                ValueType::I64,
                ValueType::F32,
                ValueType::F64
            ])))
            .to_wat(&ToWatParams::default()),
            "(result i32 i64 f32 f64)",
        );
    }
}
