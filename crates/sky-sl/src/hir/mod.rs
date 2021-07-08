use salsa::InternId;

pub mod lower;

mod block;
mod function;
mod module;
mod structure;

pub use block::*;
pub use function::*;
pub use module::*;
pub use structure::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct TyPath(InternId);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TyPathData {
    Root(String),
    Child(TyPath, String),
}

pub enum TyKind {
    Path(TyPath),
    Infer,
    Err,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ItemPath(InternId);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ItemPathData {
    Root(String),
    Child(ItemPath, String),
}
