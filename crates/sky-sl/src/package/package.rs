use crate::fs::PathSegment;

use super::Manifest;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Package {
    pub path: PathSegment,
    pub manifest: Manifest,
}

impl Package {
    pub fn new(path: PathSegment, manifest: Manifest) -> Self {
        Self {
            path,
            manifest,
        }
    }
}
