use super::FunctionKind;
use crate::hir::type_check::{TypeCheckError, Ty};
use rowan::{TextRange, TextSize};

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

    pub fn find_ty(&self, offset: TextSize) -> Option<Ty> {
        if !self.span.contains(offset) {
            return None;
        }
        
        for item in &self.items {
            if item.span().contains(offset) {
                return item.find_ty(offset);
            }
        }

        None
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ItemKind {
    Function(FunctionKind),
}

impl ItemKind {
    pub fn span(&self) -> TextRange {
        match self {
            ItemKind::Function(function) => function.span,
        }
    }

    pub fn find_ty(&self, offset: TextSize) -> Option<Ty> {
        match self {
            ItemKind::Function(function) => function.find_ty(offset)
        }
    }
}
