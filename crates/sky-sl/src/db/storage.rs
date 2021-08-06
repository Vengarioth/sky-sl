use super::*;

#[salsa::database(SourceDatabaseStorage, SyntaxDatabaseStorage, HirDatabaseStorage, TyDatabaseStorage)]
#[derive(Default)]
pub struct CompilerDatabase {
    storage: salsa::Storage<CompilerDatabase>,
}

impl salsa::Database for CompilerDatabase {}
