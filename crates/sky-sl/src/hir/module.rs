use super::{FunctionKind, StructureKind, lower::LowerToHirError};
use rowan::TextRange;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Module {
    pub items: Vec<ItemKind>,
    pub errors: Vec<LowerToHirError>,
    pub span: TextRange,
}

impl Module {
    pub fn new(items: Vec<ItemKind>, errors: Vec<LowerToHirError>, span: TextRange) -> Self {
        Self {
            span,
            errors,
            items,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ItemKind {
    Function(FunctionKind),
    Structure(StructureKind),
}
