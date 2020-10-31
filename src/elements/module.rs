use super::{Expr, SExpr, Section};

#[derive(Clone, Debug, PartialEq)]
pub struct Module {
    sections: Vec<Section>,
}

impl SExpr for Module {
    fn car(&self) -> String {
        "module".to_string()
    }

    fn cdr(&self) -> Vec<Expr> {
        self.sections.iter().flat_map(|s| s.exprs()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elements::*;
    use crate::{ToWat, ToWatParams};

    #[test]
    fn empty() {
        assert_eq!(
            Expr::SExpr(Box::new(Module {
                sections: Vec::new(),
            }))
            .to_wat(&ToWatParams::default()),
            "(module)",
        );
    }

    #[test]
    fn with_import_section() {
        assert_eq!(
            Expr::SExpr(Box::new(Module {
                sections: vec![
                    Section::Import(ImportSection::with_entries(vec![
                        ImportEntry {
                            module: Value::String(
                                "wasi_snapshot_preview1".to_string()
                            ),
                            name: Value::String("proc_exit".to_string()),
                            desc: ImportDesc::Func(FuncType {
                                id: Identifier::Symbolic(
                                    "__wasi_snapshot_preview1_proc_exit"
                                        .to_string()
                                ),
                                type_use: TypeUse {
                                    type_def: None,
                                    params: vec![Params::with_value_types(
                                        vec![ValueType::I32]
                                    )],
                                    results: vec![],
                                },
                            }),
                        },
                    ])),
                    Section::Function(FunctionSection::with_entries(vec![
                        Func {
                            id: Identifier::Symbolic("_start".to_string()),
                            type_use: TypeUse {
                                type_def: None,
                                params: vec![],
                                results: vec![],
                            },
                            export: Some(Export {
                                name: Name::new("_start".to_string()),
                                desc: None,
                            }),
                            instructions: vec![Instr::PlainInstr(
                                PlainInstr::Call(Call {
                                    funcidx: Identifier::Symbolic(
                                        "__wasi_snapshot_preview1_proc_exit"
                                            .to_string()
                                    ),
                                    params: vec![Instr::PlainInstr(
                                        PlainInstr::I32Const(
                                            I32Const::with_integer(
                                                Integer::Decimal(Decimal {
                                                    sign: Sign::Empty,
                                                    num: "0".to_string(),
                                                },)
                                            ),
                                        ),
                                    ),],
                                },),
                            ),],
                        },
                    ]))
                ],
            }))
            .to_wat(&ToWatParams::default()),
            r#"(module
  (import
    "wasi_snapshot_preview1"
    "proc_exit"
    (func $__wasi_snapshot_preview1_proc_exit (param i32))
  )
  (func
    $_start
    (export "_start")
    (call $__wasi_snapshot_preview1_proc_exit (i32.const 0))
  )
)"#
        );
    }
}
