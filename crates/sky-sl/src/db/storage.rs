use super::*;
use camino::Utf8PathBuf;
use std::sync::Arc;
use std::fmt;

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

impl fmt::Debug for CompilerDatabase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CompilerDatabase")
            .finish_non_exhaustive()
    }
}
