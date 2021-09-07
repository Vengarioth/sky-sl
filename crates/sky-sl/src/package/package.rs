use super::Manifest;
use crate::fs::FileId;
use std::sync::Arc;

#[derive(Debug, Eq, Clone)]
pub struct Package {
    pub file: FileId,
    pub manifest: Arc<Manifest>,
}

impl Package {
    pub fn new(file: FileId, manifest: Arc<Manifest>) -> Self {
        Self { file, manifest }
    }
}

impl PartialEq for Package {
    fn eq(&self, other: &Self) -> bool {
        if self.file != other.file {
            return false;
        }

        if Arc::ptr_eq(&self.manifest, &other.manifest) {
            return true;
        }

        self.manifest == other.manifest
    }
}
