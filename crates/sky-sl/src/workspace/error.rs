use crate::fs::FileSystemError;
use thiserror::*;

#[derive(Debug, Error)]
pub enum CompileError {
    #[error("File system error: {0}")]
    FileSystemError(#[from] FileSystemError),
}
