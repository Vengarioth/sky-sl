use camino::Utf8PathBuf;
use thiserror::*;

#[derive(Debug, Error)]
pub enum FileSystemError {
    #[error("Is not a valid file path: {0}")]
    IsNotAValidFilePath(Utf8PathBuf),

    #[error("File does not exist: {0}")]
    FileDoesNotExist(Utf8PathBuf),
}
