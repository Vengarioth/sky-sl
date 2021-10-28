use super::{Symbol, SymbolList};
use crate::{fs::FileId, hir::HirDatabase, intern::Name};
use std::sync::Arc;

pub struct SymbolListBuilder<'a> {
    current_file: FileId,
    db: &'a dyn HirDatabase,
    symbols: Vec<Arc<Symbol>>,
}

impl<'a> SymbolListBuilder<'a> {
    pub fn new(current_file: FileId, db: &'a dyn HirDatabase) -> Self {
        Self {
            current_file,
            db,
            symbols: Vec::new(),
        }
    }

    pub fn current_file(&self) -> FileId {
        self.current_file
    }

    pub fn intern_name(&self, name: String) -> Name {
        self.db.intern_name(name)
    }

    pub fn add_symbol(&mut self, symbol: Symbol) {
        self.symbols.push(Arc::new(symbol));
    }

    pub fn build(self) -> SymbolList {
        SymbolList::new(Arc::new(self.symbols))
    }
}
