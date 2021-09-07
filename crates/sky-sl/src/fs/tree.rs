use super::{FileDatabase, file::FileId};
use salsa::{InternId, InternKey};
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PathSegment(InternId);

impl InternKey for PathSegment {
    fn from_intern_id(v: InternId) -> Self {
        Self(v)
    }

    fn as_intern_id(&self) -> InternId {
        self.0
    }
}

impl PathSegment {
    pub fn parent(&self, db: &dyn FileDatabase) -> Option<PathSegment> {
        db.lookup_path_data(*self).parent()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum PathSegmentData {
    Root,
    Directory {
        name: String,
        parent: PathSegment,
    },
}

impl PathSegmentData {
    pub fn parent(&self) -> Option<PathSegment> {
        match self {
            PathSegmentData::Root => None,
            PathSegmentData::Directory { parent, .. } => Some(*parent),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DirectoryData {
    directories: HashSet<PathSegment>,
    files: HashSet<FileId>,
}

impl DirectoryData {
    pub fn new() -> Self {
        Self {
            directories: HashSet::new(),
            files: HashSet::new(),
        }
    }

    pub fn from_slices(directory_slice: &[PathSegment], file_slice: &[FileId]) -> Self {
        let mut directories = HashSet::new();
        let mut files = HashSet::new();

        for directory in directory_slice {
            directories.insert(*directory);
        }

        for file in file_slice {
            files.insert(*file);
        }

        Self {
            directories,
            files,
        }
    }

    pub fn directory_count(&self) -> usize {
        self.directories.len()
    }

    pub fn file_count(&self) -> usize {
        self.files.len()
    }
    
    pub fn directories(&self) -> impl Iterator<Item = &PathSegment> {
        self.directories.iter()
    }

    pub fn files(&self) -> impl Iterator<Item = &FileId> {
        self.files.iter()
    }

    pub fn contains_directory(&self, directory: &PathSegment) -> bool {
        self.directories.contains(directory)
    }

    pub fn contains_file(&self, file: &FileId) -> bool {
        self.files.contains(file)
    }

    pub fn with_directory(mut self, directory: PathSegment) -> Self {
        self.directories.insert(directory);
        self
    }

    pub fn without_directory(mut self, directory: PathSegment) -> Self {
        self.directories.remove(&directory);
        self
    }

    pub fn with_file(mut self, file: FileId) -> Self {
        self.files.insert(file);
        self
    }

    pub fn without_file(mut self, file: FileId) -> Self {
        self.files.remove(&file);
        self
    }
}
