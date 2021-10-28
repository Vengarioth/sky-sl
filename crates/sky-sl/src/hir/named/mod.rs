use super::{primitive::Primitive, symbol::Symbol};
use crate::{fs::FileId, hir::HirDatabase, intern::Name, syn::ast::*};
use std::{collections::HashMap, sync::Arc};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NamedItemKind {
    Symbol(Arc<Symbol>),
    Primitive(Arc<Primitive>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NamedScope {
    items: Arc<HashMap<Name, NamedItemKind>>,
}

impl NamedScope {
    pub fn new(items: Arc<HashMap<Name, NamedItemKind>>) -> Self {
        Self { items }
    }

    pub fn lookup(&self, name: Name) -> Option<NamedItemKind> {
        self.items.get(&name).cloned()
    }
}

pub struct NamedScopeBuilder<'a> {
    db: &'a dyn HirDatabase,
    items: HashMap<Name, NamedItemKind>,
}

impl<'a> NamedScopeBuilder<'a> {
    pub fn new(db: &'a dyn HirDatabase) -> Self {
        Self {
            db,
            items: HashMap::new(),
        }
    }

    pub fn add_primitives(&mut self) {
        for primitive in self.db.get_primitives().iter() {
            self.items.insert(primitive.name, NamedItemKind::Primitive(primitive.clone()));
        }
    }

    pub fn import_usings(&mut self, file: FileId) {
        let ast = self.db.get_ast(file);

        for _using in ast.tree().uses() {
            // TODO
        }
    }

    pub fn add_file_symbols(&mut self, file: FileId) {
        for symbol in self.db.get_symbols(file).iter() {
            self.items.insert(symbol.name, NamedItemKind::Symbol(symbol.clone()));
        }
    }

    pub fn build(self) -> NamedScope {
        NamedScope::new(Arc::new(self.items))
    }
}
