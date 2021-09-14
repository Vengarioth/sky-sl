use super::Symbol;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolList {
    inner: Arc<Vec<Symbol>>,
}

impl SymbolList {
    pub fn new(inner: Vec<Symbol>) -> Self {
        Self { inner: Arc::new(inner) }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Symbol> {
        self.inner.iter()
    }
}
