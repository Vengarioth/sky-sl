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

    pub fn discover_modules(&mut self, package: Package) {
        let mut root_path = self.root_path(package);

        // TODO configurable by manifest
        root_path.push("src");
        root_path.push("lib.skysl");

        let source = std::fs::read_to_string(&root_path).unwrap_or_default();

        self.set_input_file(root_path.clone(), Arc::new(source));

        let modules = self.modules(root_path);
        // let references = self.references(root_path);
    }
}
