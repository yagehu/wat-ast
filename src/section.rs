use wast::parser::{Parse, Parser, Result};

use crate::{
    Export, Expression, ExpressionParser, FuncType, GlobalType, Identifier,
    ImportDesc, MemType, TypeUse,
};

/// https://webassembly.github.io/spec/core/text/modules.html#text-module
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Section<'a> {
    Type(TypeSection<'a>),
    Import(ImportSection<'a>),
    Function(FunctionSection<'a>),
    Memory(MemorySection<'a>),
    Global(GlobalSection<'a>),
    Data(DataSection<'a>),
}

impl<'a> Parse<'a> for Section<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        while !parser.is_empty() {
            if parser.peek2::<wast::kw::r#type>() {
                return Ok(Self::Type(parser.parse()?));
            } else if parser.peek2::<wast::kw::import>() {
                return Ok(Self::Import(parser.parse()?));
            } else if parser.peek2::<wast::kw::func>() {
                return Ok(Self::Function(parser.parse()?));
            } else if parser.peek2::<wast::kw::memory>() {
                return Ok(Self::Memory(parser.parse()?));
            } else if parser.peek2::<wast::kw::global>() {
                return Ok(Self::Global(parser.parse()?));
            } else if parser.peek2::<wast::kw::data>() {
                return Ok(Self::Data(parser.parse()?));
            } else {
                return Err(parser.error("unexpected section"));
            }
        }

        Err(parser.error("empty section"))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeSection<'a> {
    pub entries: Vec<TypeSectionEntry<'a>>,
}

impl<'a> Parse<'a> for TypeSection<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let mut entries = Vec::new();

        while !parser.is_empty() {
            entries.push(parser.parens(|p| p.parse::<TypeSectionEntry>())?);

            if !parser.peek2::<wast::kw::r#type>() {
                break;
            }
        }

        Ok(Self { entries })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeSectionEntry<'a> {
    pub id: Option<Identifier<'a>>,
    pub func_type: FuncType,
}

impl<'a> Parse<'a> for TypeSectionEntry<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        parser.parse::<wast::kw::r#type>()?;

        let id = parser.parse::<Option<Identifier>>()?;
        let func_type = parser.parens(|p| p.parse::<FuncType>())?;

        Ok(Self { id, func_type })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportSection<'a> {
    pub entries: Vec<ImportSectionEntry<'a>>,
}

impl<'a> Parse<'a> for ImportSection<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let mut entries = Vec::new();

        while !parser.is_empty() {
            entries.push(parser.parens(ImportSectionEntry::parse)?);

            if !parser.peek2::<wast::kw::import>() {
                break;
            }
        }

        Ok(Self { entries })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportSectionEntry<'a> {
    pub module: &'a str,
    pub name: &'a str,
    pub desc: ImportDesc<'a>,
}

impl<'a> Parse<'a> for ImportSectionEntry<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        parser.parse::<wast::kw::import>()?;

        let module = parser.parse::<&str>()?;
        let name = parser.parse::<&str>()?;
        let desc = parser.parens(ImportDesc::parse)?;

        Ok(Self { module, name, desc })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionSection<'a> {
    pub entries: Vec<FunctionSectionEntry<'a>>,
}

impl<'a> Parse<'a> for FunctionSection<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let mut entries = Vec::new();

        while !parser.is_empty() {
            entries.push(parser.parens(|p| p.parse::<FunctionSectionEntry>())?);

            if !parser.peek2::<wast::kw::func>() {
                break;
            }
        }

        Ok(Self { entries })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionSectionEntry<'a> {
    pub id: Identifier<'a>,
    pub export: Option<Export<'a>>,
    pub type_use: TypeUse<'a>,
    pub exprs: Vec<Expression<'a>>,
}

impl<'a> Parse<'a> for FunctionSectionEntry<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        parser.parse::<wast::kw::func>()?;

        let id = parser.parse::<Identifier>()?;
        let mut export = None;

        if parser.peek2::<wast::kw::export>() {
            export = Some(parser.parens(Export::parse)?);
        }

        let type_use = parser.parse::<TypeUse>()?;
        let expressions_parser = ExpressionParser::default();
        let exprs = expressions_parser.parse(parser)?;

        Ok(Self {
            id,
            export,
            type_use,
            exprs,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemorySection<'a> {
    pub entries: Vec<MemorySectionEntry<'a>>,
}

impl<'a> Parse<'a> for MemorySection<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let mut entries = Vec::new();

        while !parser.is_empty() {
            entries.push(parser.parens(|p| p.parse::<MemorySectionEntry>())?);

            if !parser.peek2::<wast::kw::memory>() {
                break;
            }
        }

        Ok(Self { entries })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemorySectionEntry<'a> {
    pub id: Option<Identifier<'a>>,
    pub export: Option<Export<'a>>,
    pub mem_type: MemType<'a>,
}

impl<'a> Parse<'a> for MemorySectionEntry<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        parser.parse::<wast::kw::memory>()?;

        let id = parser.parse::<Option<Identifier>>()?;
        let mut export = None;

        if parser.peek2::<wast::kw::export>() {
            export = Some(parser.parens(Export::parse)?);
        }

        let mem_type = parser.parse::<MemType>()?;

        Ok(Self {
            id,
            export,
            mem_type,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobalSection<'a> {
    pub entries: Vec<GlobalSectionEntry<'a>>,
}

impl<'a> Parse<'a> for GlobalSection<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let mut entries = Vec::new();

        while !parser.is_empty() {
            entries.push(parser.parens(|p| p.parse::<GlobalSectionEntry>())?);

            if !parser.peek2::<wast::kw::global>() {
                break;
            }
        }

        Ok(Self { entries })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobalSectionEntry<'a> {
    pub id: Option<Identifier<'a>>,
    pub export: Option<Export<'a>>,
    pub global_type: GlobalType,
    pub expr: Expression<'a>,
}

impl<'a> Parse<'a> for GlobalSectionEntry<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        parser.parse::<wast::kw::global>()?;

        let id = parser.parse::<Option<Identifier>>()?;
        let mut export = None;

        if parser.peek2::<wast::kw::export>() {
            export = Some(parser.parens(Export::parse)?);
        }

        let global_type = parser.parse::<GlobalType>()?;
        let mut exprs = ExpressionParser::default().parse(parser)?;

        if exprs.len() != 1 {
            return Err(parser.error("only one expr is expected"));
        }

        let expr = exprs.pop().unwrap();

        Ok(Self {
            id,
            export,
            global_type,
            expr,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataSection<'a> {
    pub entries: Vec<DataSectionEntry<'a>>,
}

impl<'a> Parse<'a> for DataSection<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let mut entries = Vec::new();

        while !parser.is_empty() {
            entries.push(parser.parens(DataSectionEntry::parse)?);

            if !parser.peek2::<wast::kw::data>() {
                break;
            }
        }

        Ok(Self { entries })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Offset<'a> {
    Folded(Expression<'a>),
    Unfolded(Expression<'a>),
}

impl<'a> Parse<'a> for Offset<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let mut exprs;
        let is_folded;

        if parser.peek2::<wast::kw::offset>() {
            exprs = parser.parens(|p| {
                p.parse::<wast::kw::offset>()?;

                Ok(ExpressionParser::default().parse(parser)?)
            })?;
            is_folded = true;
        } else {
            exprs = ExpressionParser::default().parse(parser)?;
            is_folded = false;
        }

        if exprs.len() == 0 {
            return Err(parser.error("init_expr is empty"));
        }

        if exprs.len() > 1 {
            return Err(parser.error("only one init_expr operator is expected"));
        }

        if is_folded {
            Ok(Self::Folded(exprs.pop().unwrap()))
        } else {
            Ok(Self::Unfolded(exprs.pop().unwrap()))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataString<'a> {
    pub strs: Vec<&'a str>,
}

impl<'a> Parse<'a> for DataString<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let mut strs = Vec::new();

        while !parser.is_empty() {
            strs.push(parser.parse::<&str>()?);
        }

        Ok(Self { strs })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataSectionEntry<'a> {
    pub id: Option<Identifier<'a>>,
    pub offset: Offset<'a>,
    pub data_string: DataString<'a>,
}

impl<'a> Parse<'a> for DataSectionEntry<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        parser.parse::<wast::kw::data>()?;

        let id = parser.parse::<Option<Identifier>>()?;
        let offset = parser.parse::<Offset>()?;
        let data_string = parser.parse::<DataString>()?;

        Ok(Self {
            id,
            offset,
            data_string,
        })
    }
}
