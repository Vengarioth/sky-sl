use super::*;
use crate::fs::db::*;
use crate::package::*;
use std::fmt;

#[salsa::database(
    FileDatabaseStorage,
    PackageDatabaseStorage,
    ManifestDatabaseStorage,
    SourceDatabaseStorage,
    SyntaxDatabaseStorage,
    ModuleDatabaseStorage,
    HirDatabaseStorage,
    TyDatabaseStorage
)]
#[derive(Default)]
pub struct CompilerDatabase {
    storage: salsa::Storage<CompilerDatabase>,
}

impl salsa::Database for CompilerDatabase {}

impl fmt::Debug for CompilerDatabase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CompilerDatabase")
            .finish_non_exhaustive()
    }
}
