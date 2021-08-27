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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum PathSegmentData {
    Root,
    Directory {
        name: String,
        parent: PathSegment,
    },
    File {
        name: String,
        parent: PathSegment,
    },
}

impl PathSegmentData {
    pub fn parent(&self) -> Option<PathSegment> {
        match self {
            PathSegmentData::Root => None,
            PathSegmentData::Directory { parent, .. } => Some(*parent),
            PathSegmentData::File { parent, .. } => Some(*parent),
        }
    }

    pub fn is_file(&self) -> bool {
        match self {
            PathSegmentData::Root => false,
            PathSegmentData::Directory { .. } => false,
            PathSegmentData::File { .. } => true,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DirectoryData {
    children: HashSet<PathSegment>,
}

impl DirectoryData {
    pub fn new() -> Self {
        Self {
            children: HashSet::new(),
        }
    }

    pub fn from_slice(slice: &[PathSegment]) -> Self {
        let mut children = HashSet::new();
        for item in slice {
            children.insert(*item);
        }

        Self {
            children,
        }
    }

    pub fn contains(&self, segment: &PathSegment) -> bool {
        self.children.contains(segment)
    }

    pub fn with_segment(&self, segment: PathSegment) -> Self {
        let mut children = self.children.clone();
        children.insert(segment);
        Self {
            children,
        }
    }
}
