pub(crate) mod db;
mod tree;
mod file;
mod error;

pub use db::{FileDatabase, initialize_fs, insert_file, remove_file, lookup_file};
pub use tree::{PathSegment};
pub use error::*;
pub use file::{FileId};

#[cfg(test)]
mod tests;
