use salsa::{InternId, InternKey};

#[salsa::query_group(InternDatabaseStorage)]
pub trait InternDatabase: salsa::Database {
    #[salsa::interned]
    fn intern_name(&self, name: String) -> Name;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Name(InternId);

impl InternKey for Name {
    fn from_intern_id(v: InternId) -> Self {
        Self(v)
    }

    fn as_intern_id(&self) -> InternId {
        self.0
    }
}
