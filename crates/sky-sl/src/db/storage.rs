use super::*;
use camino::Utf8PathBuf;
use std::sync::Arc;

#[salsa::database(
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

impl CompilerDatabase {
    pub fn add_package(&self, root: Utf8PathBuf) -> Package {
        self.intern_package(Arc::new(PackageData {
            root,
        }))
    }
}
