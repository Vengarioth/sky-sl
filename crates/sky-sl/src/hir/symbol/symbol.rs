use rowan::TextRange;

use crate::fs::FileId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SymbolKind {
    Module,
    Function,
    Struct,
    Field,
    Value,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Symbol {
    pub name: String,
    pub file: FileId,
    pub span: TextRange,
    pub selection_span: TextRange,
    pub kind: SymbolKind,
    pub children: Option<Vec<Symbol>>,
}

impl Symbol {
    pub fn new(
        name: String,
        file: FileId,
        span: TextRange,
        selection_span: TextRange,
        kind: SymbolKind,
        children: Option<Vec<Symbol>>,
    ) -> Self {
        Self {
            name,
            file,
            span,
            selection_span,
            kind,
            children,
        }
    }

    pub fn reference(&self) -> SymbolReference {
        SymbolReference::new(self.name.to_owned(), self.file)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SymbolReference {
    pub name: String,
    pub file: FileId,
}

impl SymbolReference {
    pub fn new(name: String, file: FileId) -> Self {
        Self { name, file }
    }
}
