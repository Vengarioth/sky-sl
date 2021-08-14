use camino::{Utf8Path, Utf8PathBuf};
use serde::Deserialize;
use thiserror::Error;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Hash, Deserialize)]
pub struct WorkspaceManifest {
    pub project: WorkspaceManifestProject,
}

#[derive(Debug, Eq, PartialEq, Hash, Deserialize)]
pub struct WorkspaceManifestProject {
    pub name: String,
    pub path: Option<String>,
}

impl WorkspaceManifestProject {
    pub fn path(&self) -> Utf8PathBuf {
        if let Some(path) = &self.path {
            Utf8PathBuf::from_str(path).unwrap()
        } else {
            Utf8PathBuf::from_str("src/lib.skysl").unwrap()
        }
    }
}

impl WorkspaceManifest {
    pub fn from_file(path: &Utf8Path) -> Result<Self, WorkspaceManifestError> {
        let file_contents = std::fs::read_to_string(path).map_err(|_| WorkspaceManifestError::ManifestNotFound)?;
        Self::from_str(&file_contents)
    }
}

impl FromStr for WorkspaceManifest {
    type Err = WorkspaceManifestError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let manifest = toml::from_str::<WorkspaceManifest>(s).map_err(|_| WorkspaceManifestError::ManifestParseError)?;
        Ok(manifest)
    }
}

#[derive(Error, Debug)]
pub enum WorkspaceManifestError {
    #[error("Manifest not found")]
    ManifestNotFound,

    #[error("Could not parse manifest")]
    ManifestParseError,
}
