use camino::{Utf8Path, Utf8PathBuf};

use crate::db::*;
use crate::syn::{Parse, cst::LineIndex, ast::Root};
use crate::hir::type_check::Ty;
use std::sync::{Arc, Mutex};

mod fs;
mod error;
mod package;

pub use error::*;
pub use package::*;

struct Inner {
    root: Utf8PathBuf,
    manifest: Utf8PathBuf,
    db: Mutex<CompilerDatabase>,
}

impl std::fmt::Debug for Inner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Inner")
            .field("root", &self.root)
            .field("manifest", &self.manifest)
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
        let manifest = root.join("skysl.package");
        Self {
            inner: Arc::new(Inner {
                root,
                manifest,
                db: Mutex::new(CompilerDatabase::default()),
            }),
        }
    }

    /// Returns the absolute path to the package manifest
    pub fn package_manifest(&self) -> &Utf8Path {
        &self.inner.manifest
    }

    /// Returns the absolute path to the package root
    pub fn package_root(&self) -> &Utf8Path {
        &self.inner.root
    }

    /// Notify the workspace that a file has changed
    pub fn update_file(&mut self, path: Utf8PathBuf, contents: Arc<String>) {
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

    /// deprecated
    pub fn document_symbols(&self, path: &Utf8Path) -> Result<Parse<Root>, ()> {
        let input = std::fs::read_to_string(path).expect("could not read file to string");
        let token = crate::lexer::tokenize(&input);
        Ok(crate::parser::parse(&token, &input))
    }

    /// deprecated
    pub fn get_line_index(&self, path: Utf8PathBuf) -> Result<LineIndex, ()> {
        let input = std::fs::read_to_string(path).expect("could not read file to string");
        Ok(LineIndex::new(&input))
    }
}

impl Clone for Workspace {
    fn clone(&self) -> Self {
        Workspace {
            inner: Arc::clone(&self.inner)
        }
    }
}
