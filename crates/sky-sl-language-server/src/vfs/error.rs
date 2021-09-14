use camino::Utf8PathBuf;
use thiserror::*;

#[derive(Debug, Error)]
pub enum VirtualFileSystemError {
    #[error("Invalid Root: {0}")]
    InvalidRoot(Utf8PathBuf),

    #[error("Cannot access file: {0}")]
    CannotAccessFile(Utf8PathBuf),

    #[error("Root is not a directory: {0}")]
    RootIsNotADir(Utf8PathBuf),

    #[error("Root is not absolute: {0}")]
    RootIsNotAbsolute(Utf8PathBuf),

    #[error("File does not exist: {0}")]
    FileDoesNotExist(Utf8PathBuf),
}
