use super::{FunctionKind, LayoutKind, StructKind};
use crate::{hir::{lower::LowerToHirError, named::NamedItemKind}, text::Locate};
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

impl Locate for Module {
    type Item = NamedItemKind;

    fn locate(&self, offset: rowan::TextSize) -> Option<Self::Item> {
        if !self.span.contains(offset) {
            return None;
        }

        for item in &self.items {
            if let Some(item) = item.locate(offset) {
                return Some(item);
            }
        }
        None
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ItemKind {
    Function(FunctionKind),
    Struct(StructKind),
    Layout(LayoutKind),
}

impl Locate for ItemKind {
    type Item = NamedItemKind;

    fn locate(&self, offset: rowan::TextSize) -> Option<Self::Item> {
        match self {
            ItemKind::Function(f) => f.locate(offset),
            ItemKind::Struct(s) => s.locate(offset),
            ItemKind::Layout(l) => l.locate(offset),
        }
    }
}
