use super::{FunctionKind};
use crate::hir::type_check::{TypeCheckError};
use rowan::TextRange;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Module {
    pub items: Vec<ItemKind>,
    pub errors: Vec<TypeCheckError>,
    pub span: TextRange,
}

impl Module {
    pub fn new(items: Vec<ItemKind>, errors: Vec<TypeCheckError>, span: TextRange) -> Self {
        Self {
            items,
            errors,
            span,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ItemKind {
    Function(FunctionKind),
}
