use crate::{fs::FileId, syn::db::SyntaxDatabase};
use super::symbol::{SymbolList, find_symbols};

#[salsa::query_group(HirDatabaseStorage)]
pub trait HirDatabase: SyntaxDatabase {
    fn get_symbols(&self, file: FileId) -> SymbolList;
    fn get_hir(&self, file: FileId) -> super::untyped::Module;
    fn get_typed_hir(&self, file: FileId) -> super::typed::Module;
}

fn get_hir(db: &dyn HirDatabase, file: FileId) -> super::untyped::Module {
    let ast = db.get_ast(file);
    super::lower::lower_ast_to_hir(file, db, &ast.tree())
}

fn get_typed_hir(db: &dyn HirDatabase, file: FileId) -> super::typed::Module {
    let hir = db.get_hir(file);
    let mut env = super::type_check::Env::empty();
    super::type_check::infer_module(&hir, &mut env)
}

fn get_symbols(db: &dyn HirDatabase, file: FileId) -> SymbolList {
    let ast = db.get_ast(file);
    find_symbols(file, ast.tree())
}
