use super::LowerToHirError;
use crate::{fs::FileId, hir::{HirDatabase, named::{NamedItemKind, NamedScope}, untyped::{ItemKind, Module}}, intern::Name, syn::ast::*};

pub struct HirModuleBuilder<'a> {
    db: &'a dyn HirDatabase,
    current_file: FileId,
    items: Vec<ItemKind>,
    diagnostics: Vec<LowerToHirError>,
    scope: NamedScope,
}

impl<'a> HirModuleBuilder<'a> {
    pub fn new(db: &'a dyn HirDatabase, current_file: FileId) -> Self {
        let scope = db.get_local_scope(current_file);

        Self {
            db,
            current_file,
            items: Vec::new(),
            diagnostics: Vec::new(),
            scope,
        }
    }

    pub fn current_file(&self) -> FileId {
        self.current_file
    }

    pub fn intern_name(&self, name: String) -> Name {
        self.db.intern_name(name)
    }

    pub fn lookup_item(&self, name: Name) -> Option<NamedItemKind> {
        self.scope.lookup(name)
    }

    pub fn lookup_item_in(&self, name: Name, file_id: FileId) -> Option<NamedItemKind> {
        let scope = self.db.get_import_scope(file_id);
        scope.lookup(name)
    }

    pub fn lookup_module_file(&self, current_module: FileId, name: Name) -> Option<FileId> {
        self.db.child_module(current_module, name)
    }

    pub fn add_item(&mut self, item: ItemKind) {
        self.items.push(item);
    }

    pub fn add_diagnostic(&mut self, diagnostic: LowerToHirError) {
        self.diagnostics.push(diagnostic);
    }

    pub fn build(self) -> Module {
        let ast = self.db.get_ast(self.current_file).tree();
        Module::new(self.items, self.diagnostics, ast.syntax().text_range())
    }
}
