use serde::Deserialize;
use thiserror::Error;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub package: PackageManifest,
}

#[derive(Debug, Deserialize)]
pub struct PackageManifest {
    pub name: String,
    pub path: Option<String>,
}

impl FromStr for Manifest {
    type Err = ManifestParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let manifest = toml::from_str::<Self>(s).map_err(|_| ManifestParseError::ParseError)?;
        Ok(manifest)
    }
}

#[derive(Error, Debug)]
pub enum ManifestParseError {
    #[error("Parse Error")]
    ParseError,
}
