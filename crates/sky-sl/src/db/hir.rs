use crate::hir::{untyped, lower};
use camino::Utf8PathBuf;

use super::*;

#[salsa::query_group(HirDatabaseStorage)]
pub trait HirDatabase: ModuleDatabase {
    fn hir(&self, name: Utf8PathBuf) -> untyped::Module;
}

fn hir(db: &dyn HirDatabase, name: Utf8PathBuf) -> untyped::Module {
    let ast = db.ast(name);
    let module = lower::lower_ast_to_hir(&ast.tree());
    module
}
