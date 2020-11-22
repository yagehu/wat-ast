use wast::parser::{Parse, Parser, Result};

use crate::{
    Export, Expression, ExpressionParser, FuncType, GlobalType, ImportDesc,
    Index, MemType, TypeUse,
};

/// https://webassembly.github.io/spec/core/text/modules.html#text-module
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Section {
    Type(TypeSection),
    Import(ImportSection),
    Function(FunctionSection),
    Memory(MemorySection),
    Global(GlobalSection),
    Data(DataSection),
}

impl Parse<'_> for Section {
    fn parse(parser: Parser<'_>) -> Result<Self> {
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
pub struct TypeSection {
    pub entries: Vec<TypeSectionEntry>,
}

impl Parse<'_> for TypeSection {
    fn parse(parser: Parser<'_>) -> Result<Self> {
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
pub struct TypeSectionEntry {
    pub idx: Option<Index>,
    pub func_type: FuncType,
}

impl Parse<'_> for TypeSectionEntry {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        parser.parse::<wast::kw::r#type>()?;

        let idx = parser.parse::<Option<Index>>()?;
        let func_type = parser.parens(|p| p.parse::<FuncType>())?;

        Ok(Self { idx, func_type })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportSection {
    pub entries: Vec<ImportSectionEntry>,
}

impl Parse<'_> for ImportSection {
    fn parse(parser: Parser<'_>) -> Result<Self> {
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
pub struct ImportSectionEntry {
    pub module: String,
    pub name: String,
    pub desc: ImportDesc,
}

impl Parse<'_> for ImportSectionEntry {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        parser.parse::<wast::kw::import>()?;

        let module = parser.parse::<String>()?;
        let name = parser.parse::<String>()?;
        let desc = parser.parens(ImportDesc::parse)?;

        Ok(Self { module, name, desc })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionSection {
    pub entries: Vec<FunctionSectionEntry>,
}

impl Parse<'_> for FunctionSection {
    fn parse(parser: Parser<'_>) -> Result<Self> {
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
pub struct FunctionSectionEntry {
    pub idx: Index,
    pub export: Option<Export>,
    pub type_use: TypeUse,
    pub exprs: Vec<Expression>,
}

impl Parse<'_> for FunctionSectionEntry {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        parser.parse::<wast::kw::func>()?;

        let idx = parser.parse::<Index>()?;
        let mut export = None;

        if parser.peek2::<wast::kw::export>() {
            export = Some(parser.parens(Export::parse)?);
        }

        let type_use = parser.parse::<TypeUse>()?;
        let expressions_parser = ExpressionParser::default();
        let exprs = expressions_parser.parse(parser)?;

        Ok(Self {
            idx,
            export,
            type_use,
            exprs,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemorySection {
    pub entries: Vec<MemorySectionEntry>,
}

impl Parse<'_> for MemorySection {
    fn parse(parser: Parser<'_>) -> Result<Self> {
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
pub struct MemorySectionEntry {
    pub idx: Option<Index>,
    pub export: Option<Export>,
    pub mem_type: MemType,
}

impl Parse<'_> for MemorySectionEntry {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        parser.parse::<wast::kw::memory>()?;

        let idx = parser.parse::<Option<Index>>()?;
        let mut export = None;

        if parser.peek2::<wast::kw::export>() {
            export = Some(parser.parens(Export::parse)?);
        }

        let mem_type = parser.parse::<MemType>()?;

        Ok(Self {
            idx,
            export,
            mem_type,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobalSection {
    pub entries: Vec<GlobalSectionEntry>,
}

impl Parse<'_> for GlobalSection {
    fn parse(parser: Parser<'_>) -> Result<Self> {
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
pub struct GlobalSectionEntry {
    pub idx: Option<Index>,
    pub export: Option<Export>,
    pub global_type: GlobalType,
    pub expr: Expression,
}

impl Parse<'_> for GlobalSectionEntry {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        parser.parse::<wast::kw::global>()?;

        let idx = parser.parse::<Option<Index>>()?;
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
            idx,
            export,
            global_type,
            expr,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataSection {
    pub entries: Vec<DataSectionEntry>,
}

impl Parse<'_> for DataSection {
    fn parse(parser: Parser<'_>) -> Result<Self> {
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
pub enum Offset {
    Folded(Expression),
    Unfolded(Expression),
}

impl Parse<'_> for Offset {
    fn parse(parser: Parser<'_>) -> Result<Self> {
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
pub struct DataString {
    pub strs: Vec<String>,
}

impl Parse<'_> for DataString {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        let mut strs = Vec::new();

        while !parser.is_empty() {
            strs.push(parser.parse::<String>()?);
        }

        Ok(Self { strs })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataSectionEntry {
    pub idx: Option<Index>,
    pub offset: Offset,
    pub data_string: DataString,
}

impl Parse<'_> for DataSectionEntry {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        parser.parse::<wast::kw::data>()?;

        let idx = parser.parse::<Option<Index>>()?;
        let offset = parser.parse::<Offset>()?;
        let data_string = parser.parse::<DataString>()?;

        Ok(Self {
            idx,
            offset,
            data_string,
        })
    }
}
