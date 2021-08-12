use super::{SyntaxDatabase, Package};
use crate::{package::ModuleTree, syn::ast::{AstNode, ModuleItemOwner}};
use camino::Utf8PathBuf;
use salsa::{InternId, InternKey};
use std::sync::Arc;

#[salsa::query_group(ModuleDatabaseStorage)]
pub trait ModuleDatabase: SyntaxDatabase {
    #[salsa::interned]
    fn intern_path_data(&self, data: ModulePathData) -> ModulePath;

    #[salsa::invoke(ModuleTree::from_package)]
    fn module_tree(&self, package: Package) -> Arc<ModuleTree>;

    #[salsa::input]
    fn module(&self, path: ModulePath) -> Arc<Module>;

    fn modules(&self, path: Utf8PathBuf) -> Arc<Vec<Module>>;
}

fn modules(db: &dyn ModuleDatabase, path: Utf8PathBuf) -> Arc<Vec<Module>> {
    let ast = db.ast(path);
    for item in ast.tree().module_items() {
        dbg!(item.syntax());
    }
    
    let mut modules = Vec::new();
    // TODO
    Arc::new(modules)
}

#[derive(Debug, Eq, PartialEq)]
pub struct Module {
    pub source: String,
}

impl Module {
    pub fn new(source: String) -> Self {
        Self {
            source,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ModulePath(InternId);

impl ModulePath {
    pub fn lookup_file_path(&self, db: &dyn ModuleDatabase) -> Utf8PathBuf {
        let mut path = Utf8PathBuf::new();
        self.lookup_file_path_recursive(db, &mut path);

        path
    }

    fn lookup_file_path_recursive(&self, db: &dyn ModuleDatabase, path: &mut Utf8PathBuf) {
        let data = db.lookup_intern_path_data(*self);

        match data {
            ModulePathData::Module(child, name) => {
                child.lookup_file_path_recursive(db, path);
                path.push(name);
            },
            ModulePathData::Package(package) => {
                path.push(db.root_path(package));
            },
        }
    }
}

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
    Package(Package),
    Module(ModulePath, String),
}

#[cfg(test)]
mod tests {
    use crate::db::*;
    use camino::Utf8PathBuf;
    use std::str::FromStr;
    use std::sync::Arc;

    #[test]
    fn it_works() {
        let db = CompilerDatabase::default();

        let package = db.intern_package(Arc::new(PackageData {
            root: Utf8PathBuf::from_str("C:\\foo\\bar").unwrap(),
        }));

        let root = db.intern_path_data(ModulePathData::Package(package));
        let module = db.intern_path_data(ModulePathData::Module(root, "module".to_string()));

        db.module(module);
    }
}
