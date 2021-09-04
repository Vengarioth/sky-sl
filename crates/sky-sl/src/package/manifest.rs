use serde::Deserialize;
use thiserror::Error;
use std::str::FromStr;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct Manifest {
    pub package: PackageManifest,
}

impl Manifest {
    pub fn empty() -> Self {
        Self {
            package: PackageManifest {
                name: "empty".to_string(),
                path: None,
            }
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
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
