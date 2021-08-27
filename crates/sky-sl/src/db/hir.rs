use crate::hir::{untyped, lower};
use camino::Utf8PathBuf;

use super::*;

#[salsa::query_group(HirDatabaseStorage)]
pub trait HirDatabase: ModuleDatabase {
    fn hir(&self, name: Utf8PathBuf) -> untyped::Module;

    fn module_hir(&self, path: ModulePath) -> untyped::Module;
}

fn hir(db: &dyn HirDatabase, name: Utf8PathBuf) -> untyped::Module {
    let ast = db.ast(name);
    let module = lower::lower_ast_to_hir(&ast.tree());
    module
}

fn module_hir(db: &dyn HirDatabase, path: ModulePath) -> untyped::Module {
    let file_path = db.module_file_path(path);
    let ast = db.ast(file_path);
    let module = lower::lower_ast_to_hir(&ast.tree());
    module
}
