use crate::{hir::HirDatabase, intern::Name};
use std::sync::Arc;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum PrimitiveKind {
    Boolean,
    Integer,
    FloatingPoint,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Primitive {
    pub name: Name,
    pub kind: PrimitiveKind,
}

impl Primitive {
    pub fn new(name: Name, kind: PrimitiveKind) -> Self {
        Self { name, kind }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PrimitiveList {
    inner: Arc<Vec<Arc<Primitive>>>,
}

impl PrimitiveList {
    pub fn new(inner: Arc<Vec<Arc<Primitive>>>) -> Self {
        Self { inner }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Arc<Primitive>> {
        self.inner.iter()
    }
}

pub struct PrimitiveListBuilder<'a> {
    db: &'a dyn HirDatabase,
    inner: Vec<Arc<Primitive>>,
}

impl<'a> PrimitiveListBuilder<'a> {
    pub fn new(db: &'a dyn HirDatabase) -> Self {
        Self {
            db,
            inner: Vec::new(),
        }
    }

    pub fn add_primitive(&mut self, name: &str, kind: PrimitiveKind) {
        let name = self.db.intern_name(name.to_string());
        self.inner.push(Arc::new(Primitive {
            name,
            kind,
        }));
    }

    pub fn build(self) -> PrimitiveList {
        PrimitiveList::new(Arc::new(self.inner))
    }
}
