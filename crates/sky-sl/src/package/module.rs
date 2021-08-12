use crate::db::*;
use camino::{Utf8PathBuf, Utf8Path};
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq)]
pub struct ModuleTreeItem {
    path: ModulePath,
    file_path: Utf8PathBuf,
    children: Vec<ModuleTreeItem>,
}

impl ModuleTreeItem {
    fn insert_modules(&self, db: &mut dyn ModuleDatabase) {
        // TODO diagnostics if file does not exist
        let source = std::fs::read_to_string(&self.file_path).unwrap_or_default();
        db.set_module(self.path, Arc::new(Module {
            source,
        }));
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ModuleTree {
    package: Package,
    root: ModuleTreeItem,
}

impl ModuleTree {
    pub fn from_package(db: &dyn ModuleDatabase, package: Package) -> Arc<ModuleTree> {
        // TODO traverse entire module graph

        let mut root_path = db.root_path(package);
        root_path.push("src");
        root_path.push("lib.skysl");

        let root = db.intern_path_data(ModulePathData::Package(package));
        let root_module = db.intern_path_data(ModulePathData::Module(root, "lib".to_string()));

        Arc::new(ModuleTree {
            package,
            root: ModuleTreeItem {
                path: root_module,
                file_path: root_path,
                children: vec![],
            }
        })
    }

    pub fn insert_modules(&self, db: &mut dyn ModuleDatabase) {
        self.root.insert_modules(db);
    }

    pub fn find(&self, db: &dyn ModuleDatabase, path: ModulePath) -> Option<&ModuleTreeItem> {
        match db.lookup_intern_path_data(path) {
            ModulePathData::Module(child, _) => {
                if let Some(tree_item) = self.find(db, child) {
                    tree_item.children.iter().find(|c| c.path == child)
                } else {
                    None
                }
            },
            ModulePathData::Package(package) => {
                if package == self.package {
                    Some(&self.root)
                } else {
                    None
                }
            },
        }
    }

    pub fn find_by_file_path(&self, path: &Utf8Path) -> Option<&ModuleTreeItem> {
        find_by_file_path_recursive(path, &self.root)
    }
}

fn find_by_file_path_recursive<'a, 'b>(path: &'a Utf8Path, module: &'b ModuleTreeItem) -> Option<&'b ModuleTreeItem> {
    if module.file_path == path {
        return Some(module);
    }

    for child in &module.children {
        if let Some(module) = find_by_file_path_recursive(path, child) {
            return Some(module);
        }
    }

    None
}
