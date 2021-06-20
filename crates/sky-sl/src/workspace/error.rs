use thiserror::*;

#[derive(Debug, Error)]
pub enum WorkspaceError {
    #[error("No package root found")]
    NoPackageRootFound,
}
