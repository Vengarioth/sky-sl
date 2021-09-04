use super::FileDatabase;
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
        db.lookup_intern_path_data(*self).parent()
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

    pub fn len(&self) -> usize {
        self.children.len()
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

    pub fn without_segment(&self, segment: PathSegment) -> Self {
        let mut children = self.children.clone();
        children.remove(&segment);
        Self {
            children,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &PathSegment> {
        self.children.iter()
    }

    pub fn find_by_name(&self, child_name: &str, db: &dyn FileDatabase) -> Option<PathSegment> {
        for child in &self.children {
            match db.lookup_intern_path_data(*child) {
                PathSegmentData::Root => {},
                PathSegmentData::Directory { name, .. } => {
                    if name == child_name {
                        return Some(*child);
                    }
                },
                PathSegmentData::File { name, .. } => {
                    if name == child_name {
                        return Some(*child);
                    }
                },
            }
        }

        None
    }
}
