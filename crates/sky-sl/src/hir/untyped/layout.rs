use super::ItemPath;
use crate::{hir::named::NamedItemKind, intern::Name, text::Locate};
use rowan::TextRange;


#[derive(Debug, Eq, PartialEq, Clone)]
pub struct LayoutKind {
    pub name: Name,
    pub item: NamedItemKind,
    pub members: Vec<LayoutMember>,
    pub span: TextRange,
}

impl LayoutKind {
    pub fn new(name: Name, item: NamedItemKind, members: Vec<LayoutMember>, span: TextRange) -> Self {
        Self {
            name,
            item,
            members,
            span,
        }
    }
}

impl Locate for LayoutKind {
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
pub struct LayoutMember {
    pub name: Name,
    pub type_path: ItemPath,
    pub span: TextRange,
}

impl LayoutMember {
    pub fn new(name: Name, type_path: ItemPath, span: TextRange) -> Self {
        Self {
            name,
            type_path,
            span,
        }
    }
}
