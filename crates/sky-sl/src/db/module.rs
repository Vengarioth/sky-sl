use super::SyntaxDatabase;
use camino::Utf8PathBuf;
use salsa::{InternId, InternKey};

#[salsa::query_group(ModuleDatabaseStorage)]
pub trait ModuleDatabase: SyntaxDatabase {
    #[salsa::interned]
    fn intern_path_data(&self, data: ModulePathData) -> ModulePath;

    #[salsa::input]
    fn module_file_path(&self, module_path: ModulePath) -> Utf8PathBuf;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ModulePath(InternId);

impl InternKey for ModulePath {
    fn from_intern_id(v: InternId) -> Self {
        Self(v)
    }

    fn as_intern_id(&self) -> InternId {
        self.0
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum ModulePathData {
    Package(String),
    Module(ModulePath, String),
}
