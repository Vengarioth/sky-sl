use super::PackageDatabase;
use camino::Utf8PathBuf;
use serde::Deserialize;
use thiserror::Error;
use std::{path::Path, str::FromStr, sync::Arc};

#[salsa::query_group(ManifestDatabaseStorage)]
pub trait ManifestDatabase: PackageDatabase {
    #[salsa::input]
    fn manifest(&self, path: Utf8PathBuf) -> Arc<Manifest>;
}

#[derive(Debug, Eq, PartialEq, Hash, Deserialize)]
pub struct Manifest {
    package: ManifestPackage,
}

#[derive(Debug, Eq, PartialEq, Hash, Deserialize)]
pub struct ManifestPackage {
    name: String,
}

impl Manifest {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, ManifestError> {
        let file_contents = std::fs::read_to_string(path)?;
        Self::from_str(&file_contents)
    }

    pub fn name(&self) -> &str {
        &self.package.name
    }
}

impl FromStr for Manifest {
    type Err = ManifestError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let manifest = toml::from_str::<Manifest>(s)?;
        Ok(manifest)
    }
}

#[derive(Error, Debug)]
pub enum ManifestError {
    #[error("Io Error")]
    Io(#[from] std::io::Error),

    #[error("Toml Error")]
    Toml(#[from] toml::de::Error),
}
