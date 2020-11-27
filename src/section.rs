use wast::parser::{Parse, Parser, Result};

use crate::{
    AsAtoms, Atom, Expr, Expression, ExpressionParser, FuncType, GlobalType,
    ImportDesc, Index, InlineExport, MemType, SExpr, TypeUse,
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

impl Section {
    pub(crate) fn exprs(&self) -> Vec<Expr> {
        match self {
            Self::Type(s) => s.exprs(),
            Self::Import(s) => s.exprs(),
            Self::Function(s) => s.exprs(),
            Self::Memory(s) => s.exprs(),
            Self::Global(s) => s.exprs(),
            Self::Data(s) => s.exprs(),
        }
    }
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
    entries: Vec<TypeSectionEntry>,
}

impl TypeSection {
    pub fn with_entries(entries: Vec<TypeSectionEntry>) -> Self {
        Self { entries }
    }

    pub(crate) fn exprs(&self) -> Vec<Expr> {
        self.entries
            .iter()
            .map(|e| Expr::SExpr(Box::new(e.clone())))
            .collect()
    }
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
    idx:       Option<Index>,
    func_type: FuncType,
}

impl TypeSectionEntry {
    pub fn new(idx: Option<Index>, func_type: FuncType) -> Self {
        Self { idx, func_type }
    }
}

impl SExpr for TypeSectionEntry {
    fn car(&self) -> String {
        "type".to_owned()
    }

    fn cdr(&self) -> Vec<Expr> {
        let mut v = Vec::new();

        if let Some(ref idx) = self.idx {
            v.push(Expr::Atom(Atom::new(idx.to_string())));
        }

        v.push(Expr::SExpr(Box::new(self.func_type.clone())));

        v
    }
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
    entries: Vec<ImportSectionEntry>,
}

impl ImportSection {
    pub fn with_entries(entries: Vec<ImportSectionEntry>) -> Self {
        Self { entries }
    }

    pub(crate) fn exprs(&self) -> Vec<Expr> {
        self.entries
            .iter()
            .map(|e| Expr::SExpr(Box::new(e.clone())))
            .collect()
    }
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
    module: String,
    name:   String,
    desc:   ImportDesc,
}

impl ImportSectionEntry {
    pub fn new(module: String, name: String, desc: ImportDesc) -> Self {
        Self { module, name, desc }
    }
}

impl SExpr for ImportSectionEntry {
    fn car(&self) -> String {
        "import".to_owned()
    }

    fn cdr(&self) -> Vec<Expr> {
        let mut v = Vec::with_capacity(3);

        v.append(
            &mut self
                .module
                .as_atoms()
                .iter()
                .map(|a| Expr::Atom(a.clone()))
                .collect(),
        );
        v.append(
            &mut self
                .name
                .as_atoms()
                .iter()
                .map(|a| Expr::Atom(a.clone()))
                .collect(),
        );
        v.push(Expr::SExpr(Box::new(self.desc.clone())));

        v
    }
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
    entries: Vec<FunctionSectionEntry>,
}

impl FunctionSection {
    pub fn with_entries(entries: Vec<FunctionSectionEntry>) -> Self {
        Self { entries }
    }

    pub(crate) fn exprs(&self) -> Vec<Expr> {
        self.entries
            .iter()
            .map(|e| Expr::SExpr(Box::new(e.clone())))
            .collect()
    }
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
    idx:           Option<Index>,
    inline_export: Option<InlineExport>,
    type_use:      TypeUse,
    exprs:         Vec<Expression>,
}

impl FunctionSectionEntry {
    pub fn new(
        idx: Option<Index>,
        inline_export: Option<InlineExport>,
        type_use: TypeUse,
        exprs: Vec<Expression>,
    ) -> Self {
        Self {
            idx,
            inline_export,
            type_use,
            exprs,
        }
    }
}

impl SExpr for FunctionSectionEntry {
    fn car(&self) -> String {
        "func".to_owned()
    }

    fn cdr(&self) -> Vec<Expr> {
        let mut v = Vec::new();

        if let Some(ref idx) = self.idx {
            v.push(Expr::Atom(Atom::new(idx.to_string())));
        }

        if let Some(ref inline_export) = self.inline_export {
            v.push(Expr::SExpr(Box::new(inline_export.clone())));
        }

        v.append(&mut self.type_use.exprs());
        v.append(&mut self.exprs.iter().map(|e| e.expr()).collect());

        v
    }
}

impl Parse<'_> for FunctionSectionEntry {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        parser.parse::<wast::kw::func>()?;

        let idx = parser.parse::<Option<Index>>()?;
        let mut inline_export = None;

        if parser.peek2::<wast::kw::export>() {
            inline_export = Some(parser.parens(InlineExport::parse)?);
        }

        let type_use = parser.parse::<TypeUse>()?;
        let expressions_parser = ExpressionParser::default();
        let exprs = expressions_parser.parse(parser)?;

        Ok(Self {
            idx,
            inline_export,
            type_use,
            exprs,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemorySection {
    entries: Vec<MemorySectionEntry>,
}

impl MemorySection {
    pub fn with_entries(entries: Vec<MemorySectionEntry>) -> Self {
        Self { entries }
    }

    pub(crate) fn exprs(&self) -> Vec<Expr> {
        self.entries
            .iter()
            .map(|e| Expr::SExpr(Box::new(e.clone())))
            .collect()
    }
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
    idx:           Option<Index>,
    inline_export: Option<InlineExport>,
    mem_type:      MemType,
}

impl MemorySectionEntry {
    pub fn new(
        idx: Option<Index>,
        inline_export: Option<InlineExport>,
        mem_type: MemType,
    ) -> Self {
        Self {
            idx,
            inline_export,
            mem_type,
        }
    }
}

impl SExpr for MemorySectionEntry {
    fn car(&self) -> String {
        "memory".to_owned()
    }

    fn cdr(&self) -> Vec<Expr> {
        let mut v = Vec::new();

        if let Some(ref idx) = self.idx {
            v.push(Expr::Atom(Atom::new(idx.to_string())));
        }

        if let Some(ref inline_export) = self.inline_export {
            v.push(Expr::SExpr(Box::new(inline_export.clone())));
        }

        v.append(&mut self.mem_type.exprs());

        v
    }
}

impl Parse<'_> for MemorySectionEntry {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        parser.parse::<wast::kw::memory>()?;

        let idx = parser.parse::<Option<Index>>()?;
        let mut inline_export = None;

        if parser.peek2::<wast::kw::export>() {
            inline_export = Some(parser.parens(InlineExport::parse)?);
        }

        let mem_type = parser.parse::<MemType>()?;

        Ok(Self {
            idx,
            inline_export,
            mem_type,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobalSection {
    entries: Vec<GlobalSectionEntry>,
}

impl GlobalSection {
    pub fn with_entries(entries: Vec<GlobalSectionEntry>) -> Self {
        Self { entries }
    }

    pub(crate) fn exprs(&self) -> Vec<Expr> {
        self.entries
            .iter()
            .map(|e| Expr::SExpr(Box::new(e.clone())))
            .collect()
    }
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
    idx:           Option<Index>,
    inline_export: Option<InlineExport>,
    global_type:   GlobalType,

    /// An imported global does not have expr.
    expr: Option<Expression>,
}

impl GlobalSectionEntry {
    pub fn new(
        idx: Option<Index>,
        inline_export: Option<InlineExport>,
        global_type: GlobalType,
        expr: Option<Expression>,
    ) -> Self {
        Self {
            idx,
            inline_export,
            global_type,
            expr,
        }
    }
}

impl SExpr for GlobalSectionEntry {
    fn car(&self) -> String {
        "global".to_owned()
    }

    fn cdr(&self) -> Vec<Expr> {
        let mut v = Vec::new();

        if let Some(ref idx) = self.idx {
            v.push(Expr::Atom(Atom::new(idx.to_string())));
        }

        if let Some(ref inline_export) = self.inline_export {
            v.push(Expr::SExpr(Box::new(inline_export.clone())));
        }

        v.push(self.global_type.expr());

        if let Some(ref expr) = self.expr {
            v.push(expr.expr());
        }

        v
    }
}

impl Parse<'_> for GlobalSectionEntry {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        parser.parse::<wast::kw::global>()?;

        let idx = parser.parse::<Option<Index>>()?;
        let mut inline_export = None;

        if parser.peek2::<wast::kw::export>() {
            inline_export = Some(parser.parens(InlineExport::parse)?);
        }

        let global_type = parser.parse::<GlobalType>()?;
        let mut exprs = ExpressionParser::default().parse(parser)?;
        let expr;

        if exprs.len() == 0 {
            expr = None;
        } else {
            if exprs.len() != 1 {
                return Err(parser.error("only one expr is expected"));
            }

            expr = Some(exprs.pop().unwrap());
        }

        Ok(Self {
            idx,
            inline_export,
            global_type,
            expr,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataSection {
    entries: Vec<DataSectionEntry>,
}

impl DataSection {
    pub fn with_entries(entries: Vec<DataSectionEntry>) -> Self {
        Self { entries }
    }

    pub(crate) fn exprs(&self) -> Vec<Expr> {
        self.entries
            .iter()
            .map(|e| Expr::SExpr(Box::new(e.clone())))
            .collect()
    }
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
pub struct Offset(Expression);

impl Offset {
    pub fn new(expression: Expression) -> Self {
        Self(expression)
    }

    pub(crate) fn expr(&self) -> Expr {
        self.0.expr()
    }
}

impl Parse<'_> for Offset {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        let mut exprs = ExpressionParser::default().parse(parser)?;

        if exprs.len() == 0 {
            return Err(parser.error("init_expr is empty"));
        }

        if exprs.len() > 1 {
            return Err(parser.error("only one init_expr operator is expected"));
        }

        Ok(Self(exprs.pop().unwrap()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataString {
    strings: Vec<String>,
}

impl DataString {
    pub fn with_strings(strings: Vec<String>) -> Self {
        Self { strings }
    }

    pub(crate) fn exprs(&self) -> Vec<Expr> {
        self.strings
            .iter()
            .map(|s| Expr::Atom(Atom::new(format!(r#""{}""#, s))))
            .collect()
    }
}

impl Parse<'_> for DataString {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        let mut strings = Vec::new();

        while !parser.is_empty() {
            strings.push(parser.parse::<String>()?);
        }

        Ok(Self { strings })
    }
}

/// https://webassembly.github.io/spec/core/text/modules.html#data-segments
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataSectionEntry {
    idx:         Option<Index>,
    offset:      Offset,
    data_string: DataString,
}

impl DataSectionEntry {
    pub fn new(
        idx: Option<Index>,
        offset: Offset,
        data_string: DataString,
    ) -> Self {
        Self {
            idx,
            offset,
            data_string,
        }
    }
}

impl SExpr for DataSectionEntry {
    fn car(&self) -> String {
        "data".to_owned()
    }

    fn cdr(&self) -> Vec<Expr> {
        let mut v = Vec::new();

        if let Some(idx) = self.idx.clone() {
            v.push(Expr::Atom(Atom::new(idx.to_string())));
        }

        v.push(self.offset.expr());
        v.append(&mut self.data_string.exprs());

        v
    }
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
