use salsa::{InternId, InternKey};

use super::PathSegment;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct FileId(InternId);

impl InternKey for FileId {
    fn from_intern_id(v: InternId) -> Self {
        Self(v)
    }

    fn as_intern_id(&self) -> InternId {
        self.0
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct FileData {
    pub name: String,
    pub parent: PathSegment,
}

impl FileData {
    pub fn new(name: String, parent: PathSegment) -> Self {
        Self { name, parent }
    }
}
