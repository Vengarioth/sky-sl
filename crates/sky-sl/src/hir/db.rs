use super::{
    named::{NamedScope, NamedScopeBuilder},
    primitive::{PrimitiveKind, PrimitiveList, PrimitiveListBuilder},
    symbol::{find_symbols, SymbolList},
};
use crate::{fs::FileId, syn::db::SyntaxDatabase};

#[salsa::query_group(HirDatabaseStorage)]
pub trait HirDatabase: SyntaxDatabase {
    fn get_symbols(&self, file: FileId) -> SymbolList;
    fn get_hir(&self, file: FileId) -> super::untyped::Module;
    fn get_typed_hir(&self, file: FileId) -> super::typed::Module;
    fn get_primitives(&self) -> PrimitiveList;
    fn get_local_scope(&self, file: FileId) -> NamedScope;
    fn get_import_scope(&self, file: FileId) -> NamedScope;
}

fn get_hir(db: &dyn HirDatabase, file: FileId) -> super::untyped::Module {
    let ast = db.get_ast(file);
    super::lower::lower_ast_to_hir(file, db, &ast.tree())
}

fn get_typed_hir(db: &dyn HirDatabase, file: FileId) -> super::typed::Module {
    // let hir = db.get_hir(file);
    // let mut env = super::type_check::Env::empty();
    // super::type_check::infer_module(&hir, &mut env)
    todo!()
}

fn get_symbols(db: &dyn HirDatabase, file: FileId) -> SymbolList {
    let ast = db.get_ast(file);
    find_symbols(file, ast.tree(), db)
}

fn get_primitives(db: &dyn HirDatabase) -> PrimitiveList {
    let mut builder = PrimitiveListBuilder::new(db);

    builder.add_primitive("bool", PrimitiveKind::Boolean);

    builder.add_primitive("u8", PrimitiveKind::Integer);
    builder.add_primitive("u16", PrimitiveKind::Integer);
    builder.add_primitive("u32", PrimitiveKind::Integer);
    builder.add_primitive("u64", PrimitiveKind::Integer);

    builder.add_primitive("i8", PrimitiveKind::Integer);
    builder.add_primitive("i16", PrimitiveKind::Integer);
    builder.add_primitive("i32", PrimitiveKind::Integer);
    builder.add_primitive("i64", PrimitiveKind::Integer);

    builder.add_primitive("f16", PrimitiveKind::FloatingPoint);
    builder.add_primitive("f32", PrimitiveKind::FloatingPoint);
    builder.add_primitive("f64", PrimitiveKind::FloatingPoint);

    builder.build()
}

fn get_local_scope(db: &dyn HirDatabase, file: FileId) -> NamedScope {
    let mut builder = NamedScopeBuilder::new(db);

    builder.add_primitives();
    builder.import_usings(file);
    builder.add_file_symbols(file);

    builder.build()
}

fn get_import_scope(db: &dyn HirDatabase, file: FileId) -> NamedScope {
    let mut builder = NamedScopeBuilder::new(db);

    builder.add_file_symbols(file);

    builder.build()
}
