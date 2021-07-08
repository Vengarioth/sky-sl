use crate::hir::*;
use camino::Utf8PathBuf;

use super::*;

#[salsa::query_group(HirDatabaseStorage)]
pub trait HirDatabase: SyntaxDatabase {
    fn hir(&self, name: Utf8PathBuf) -> Module;
}

fn hir(db: &dyn HirDatabase, name: Utf8PathBuf) -> Module {
    let ast = db.ast(name);
    let module = lower::lower_ast_to_hir(&ast.tree());
    module
}
