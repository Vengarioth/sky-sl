use crate::intern::Name;

use super::{Symbol, SymbolKind};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolList {
    inner: Arc<Vec<Arc<Symbol>>>,
}

impl SymbolList {
    pub fn new(inner: Arc<Vec<Arc<Symbol>>>) -> Self {
        Self { inner: inner }
    }

    pub fn find_by_name(&self, name: Name) -> Option<Arc<Symbol>> {
        self.inner.iter().find(|s| s.name == name).cloned()
    }

    pub fn find_by_name_and_kind(&self, name: Name, kind: SymbolKind) -> Option<Arc<Symbol>> {
        self.inner
            .iter()
            .find(|s| s.name == name && s.kind == kind)
            .cloned()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Arc<Symbol>> {
        self.inner.iter()
    }
}
