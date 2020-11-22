pub use document::Document;
pub use export::Export;
pub use expr::{Expression, ExpressionParser, Instruction};
pub use import_desc::{ImportDesc, ImportDescFunc};
pub use index::{Index, Indexes, SymbolicIndex};
pub use integer::{Integer, Sign};
pub use module::Module;
pub use param::Param;
pub use result::Result;
pub use section::{
    DataSection, DataSectionEntry, FunctionSection, FunctionSectionEntry,
    GlobalSection, GlobalSectionEntry, ImportSection, ImportSectionEntry,
    MemorySection, MemorySectionEntry, Section, TypeSection, TypeSectionEntry,
};
pub use type_use::TypeUse;
pub use types::{FuncType, GlobalType, MemType, ValueType};

mod document;
mod export;
mod expr;
mod import_desc;
mod index;
mod integer;
mod module;
mod param;
mod result;
mod section;
mod type_use;
mod types;
