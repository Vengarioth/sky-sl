use super::manifest::{WorkspaceManifest, WorkspaceManifestError};
use camino::{Utf8Path, Utf8PathBuf};
use thiserror::Error;
use crate::db::*;
use std::sync::Arc;

#[derive(Debug)]
pub struct Workspace {
    manifest: WorkspaceManifest,
    db: CompilerDatabase,
}

impl Workspace {
    pub fn load_from_file(path: &Utf8Path) -> Result<Self, WorkspaceError> {
        let manifest = WorkspaceManifest::from_file(path)?;
        let db = CompilerDatabase::default();

        Ok(Self {
            manifest,
            db,
        })
    }

    /// Returns information about all projects in the current workspace
    pub fn get_projects(&self) -> Vec<ProjectInfo> {
        // currently there is only one project per workspace
        vec![ProjectInfo {
            name: self.manifest.project.name.to_string(),
            path: self.manifest.project.path().into(),
        }]
    }

    pub fn set_source(&mut self, path: Utf8PathBuf, source: String) {
        // TODO update paths and modules
        self.db.set_input_file(path, Arc::new(source));
    }

    pub fn get_ast(&self, path: &Utf8Path) -> crate::syn::Parse<crate::syn::ast::Root> {
        self.db.ast(path.to_owned())
    }

    pub fn get_source_path(&self, module_path: ModulePath) -> Utf8PathBuf {
        self.db.module_file_path(module_path)
    }

    pub(super) fn db(&self) -> &CompilerDatabase {
        &self.db
    }

    pub(super) fn db_mut(&mut self) -> &mut CompilerDatabase {
        &mut self.db
    }
}

#[derive(Error, Debug)]
pub enum WorkspaceError {
    #[error("Manifest error")]
    ManifestError(#[from] WorkspaceManifestError),
}

#[derive(Debug)]
pub struct ProjectInfo {
    /// Name of the project
    pub name: String,
    /// Path of the project's entry point, relative to the workspace manifest
    pub path: Utf8PathBuf,
}
