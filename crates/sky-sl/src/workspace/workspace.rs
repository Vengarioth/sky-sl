use super::WorkspaceError;
use crate::db::*;
use camino::Utf8PathBuf;
use std::sync::Arc;

#[derive(Debug)]
pub struct Workspace {
    root: Utf8PathBuf,
    db: CompilerDatabase,
}

impl Workspace {
    pub fn create(root: Utf8PathBuf) -> Result<Self, WorkspaceError> {
        let db = CompilerDatabase::default();
        Ok(Self {
            root,
            db,
        })
    }

    pub fn set_file_contents(&mut self, path: Utf8PathBuf, contents: Arc<String>) {
        self.db.set_input_file(path, contents);
    }
}
