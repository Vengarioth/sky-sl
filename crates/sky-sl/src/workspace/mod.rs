use camino::{Utf8Path, Utf8PathBuf};

use crate::db::*;
use crate::syn::cst::LineIndex;
use crate::hir::type_check::Ty;
use std::sync::{Arc, Mutex};

mod fs;
mod error;
mod package;
mod workspace;
mod manifest;

pub use error::*;
pub use package::*;

struct Inner {
    root: Utf8PathBuf,
    db: Mutex<CompilerDatabase>,
}

impl std::fmt::Debug for Inner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Inner")
            .field("root", &self.root)
            .finish_non_exhaustive()
    }
}

#[derive(Debug)]
pub struct Workspace {
    inner: Arc<Inner>,
}

impl Workspace {
    /// Looks for a SkySL package root in the file system by recursively looking for a package defining file upwards in the file system hierachy
    pub fn find_package_root(query: &Utf8Path) -> Result<Utf8PathBuf, WorkspaceError> {
        fs::find_package_root(query).ok_or_else(|| WorkspaceError::NoPackageRootFound)
    }

    /// Creates a new workspace with the given root path
    pub fn new(root: Utf8PathBuf) -> Self {

        let db = CompilerDatabase::default();

        Self {
            inner: Arc::new(Inner {
                root,
                db: Mutex::new(db),
            }),
        }
    }

    /// Returns the absolute path to the package root
    pub fn package_root(&self) -> &Utf8Path {
        &self.inner.root
    }

    /// Notify the workspace that a file has changed
    pub fn update_file(&mut self, path: Utf8PathBuf, contents: Arc<String>) {
        // map path to module
        dbg!(&path);
        // insert new version of module
        self.inner.db.lock().unwrap().set_input_file(path, contents);
    }

    /// Lazily build the AST for the given file
    pub fn ast(&self, path: &Utf8Path) -> Result<crate::syn::Parse<crate::syn::ast::Root>, ()> {
        Ok(self.inner.db.lock().unwrap().ast(path.into()))
    }

    /// Lazily build the line index for the given file
    pub fn line_index(&self, path: &Utf8Path) -> Result<Arc<LineIndex>, ()> {
        Ok(self.inner.db.lock().unwrap().line_index(path.into()))
    }

    pub fn type_at(&self, path: &Utf8Path, line: u32, character: u32) -> Result<Option<Ty>, ()> {
        Ok(self.inner.db.lock().unwrap().type_at(path.into(), line, character))
    }
}

impl Clone for Workspace {
    fn clone(&self) -> Self {
        Workspace {
            inner: Arc::clone(&self.inner)
        }
    }
}
