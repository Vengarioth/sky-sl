use rowan::TextRange;

use crate::{hir::named::NamedItemKind, intern::Name};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ItemPath {
    pub first_segment: ItemPathSegment,
    pub span: TextRange,
}

impl ItemPath {
    pub fn new(first_segment: ItemPathSegment, span: TextRange) -> Self {
        Self {
            first_segment,
            span,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ItemPathSegment {
    pub name: Name,
    pub item: NamedItemKind,
    pub next_segment: Option<Box<ItemPathSegment>>,
    pub span: TextRange,
}

impl ItemPathSegment {
    pub fn new(name: Name, item: NamedItemKind, next_segment: Option<Box<ItemPathSegment>>, span: TextRange) -> Self {
        Self {
            name,
            item,
            next_segment,
            span,
        }
    }
}
