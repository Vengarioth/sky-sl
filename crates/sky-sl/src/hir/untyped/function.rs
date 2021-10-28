use rowan::TextRange;
use crate::{hir::named::NamedItemKind, intern::Name, text::Locate};

use super::Block;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FunctionKind {
    pub signature: FunctionSignature,
    pub block: Block,
    pub span: TextRange,
}

impl FunctionKind {
    pub fn new(signature: FunctionSignature, block: Block, span: TextRange) -> Self {
        Self {
            signature,
            block,
            span,
        }
    }
}

impl Locate for FunctionKind {
    type Item = NamedItemKind;

    fn locate(&self, offset: rowan::TextSize) -> Option<Self::Item> {
        if !self.span.contains(offset) {
            return None;
        }

        self.signature.locate(offset)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FunctionSignature {
    pub name: Name,
    pub item: NamedItemKind,
    pub arguments: Vec<FunctionArgument>,
    pub return_type: Option<NamedItemKind>,
    pub span: TextRange,
}

impl FunctionSignature {
    pub fn new(name: Name, item: NamedItemKind, arguments: Vec<FunctionArgument>, return_type: Option<NamedItemKind>, span: TextRange) -> Self {
        Self {
            name,
            item,
            arguments,
            return_type,
            span,
        }
    }
}

impl Locate for FunctionSignature {
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
pub struct FunctionArgument {
    pub name: Name,
    pub item_type: NamedItemKind,
    pub span: TextRange,
}

impl FunctionArgument {
    pub fn new(name: Name, item_type: NamedItemKind, span: TextRange) -> Self {
        Self {
            name,
            item_type,
            span,
        }
    }
}
