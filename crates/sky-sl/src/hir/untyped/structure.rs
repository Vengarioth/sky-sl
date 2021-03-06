use crate::{hir::named::NamedItemKind, intern::Name, text::Locate};
use rowan::TextRange;

use super::ItemPath;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct StructKind {
    pub name: Name,
    pub item: NamedItemKind,
    pub members: Vec<StructMember>,
    pub span: TextRange,
}

impl StructKind {
    pub fn new(name: Name, item: NamedItemKind, members: Vec<StructMember>, span: TextRange) -> Self {
        Self {
            name,
            item,
            members,
            span,
        }
    }
}

impl Locate for StructKind {
    type Item = NamedItemKind;

    fn locate(&self, offset: rowan::TextSize) -> Option<Self::Item> {
        if self.span.contains(offset) {
            Some(self.item.clone())
        } else {
            None
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct StructMember {
    pub name: Name,
    pub item_path: ItemPath,
    pub span: TextRange,
}

impl StructMember {
    pub fn new(name: Name, item_path: ItemPath, span: TextRange) -> Self {
        Self {
            name,
            item_path,
            span,
        }
    }
}
