use super::LowerToHirError;
use crate::hir::{untyped::ItemKind, HirDatabase};

pub struct HirModuleBuilder<'a> {
    db: &'a dyn HirDatabase,
    items: Vec<ItemKind>,
    diagnostics: Vec<LowerToHirError>,
}

impl<'a> HirModuleBuilder<'a> {
    pub fn new(db: &'a dyn HirDatabase) -> Self {
        Self {
            db,
            items: Vec::new(),
            diagnostics: Vec::new(),
        }
    }

    pub fn resolve_type_name(&self, name: &str) {
        
    }

    pub fn create_function(&mut self) {}

    pub fn create_struct(&mut self) {}
}
