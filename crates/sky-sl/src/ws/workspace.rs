use crate::db::CompilerDatabase;
use crate::fs::{PathSegment, initialize_fs, insert_file, remove_file};
use crate::package::{Package, PackageDatabase};
use camino::{Utf8Path, Utf8PathBuf};
use std::sync::Arc;

#[derive(Debug)]
pub struct Workspace {
    root_path: Utf8PathBuf,
    db: CompilerDatabase,
}

impl Workspace {
    pub fn create(root_path: Utf8PathBuf) -> Self {
        let mut db = CompilerDatabase::default();
        initialize_fs(&mut db);

        Self {
            root_path,
            db,
        }
    }

    pub fn insert_file(&mut self, path: &Utf8Path, contents: Arc<String>) {
        let path = path.strip_prefix(&self.root_path).unwrap();
        dbg!(path);
        insert_file(&mut self.db, path, contents);
    }

    pub fn remove_file(&mut self, path: &Utf8Path) {
        let path = path.strip_prefix(&self.root_path).unwrap();
        remove_file(&mut self.db, path);
    }

    pub fn find_packages(&self) -> Vec<Package> {
        self.db.find_packages()
    }

    pub fn debug_fs(&self) -> crate::fs::DebugItem {
        crate::fs::debug(&self.db)
    }
}
