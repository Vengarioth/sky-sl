use rowan::TextRange;

use crate::{fs::FileId, intern::Name};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SymbolKind {
    Module,
    Function,
    Struct,
    Value,
    Layout,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Symbol {
    pub name: Name,
    pub file: FileId,
    pub span: TextRange,
    pub selection_span: TextRange,
    pub kind: SymbolKind,
    pub member: Vec<SymbolMember>,
}

impl Symbol {
    pub fn new(
        name: Name,
        file: FileId,
        span: TextRange,
        selection_span: TextRange,
        kind: SymbolKind,
        member: Vec<SymbolMember>,
    ) -> Self {
        Self {
            name,
            file,
            span,
            selection_span,
            kind,
            member,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct SymbolMember {
    pub name: Name,
    pub span: TextRange,
    pub selection_span: TextRange,
    pub kind: SymbolMemberKind,
}

impl SymbolMember {
    pub fn new(
        name: Name,
        span: TextRange,
        selection_span: TextRange,
        kind: SymbolMemberKind,
    ) -> Self {
        Self {
            name,
            span,
            selection_span,
            kind,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SymbolMemberKind {
    Field,
}
