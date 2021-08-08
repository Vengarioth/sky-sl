use rowan::TextSize;
use salsa::{InternId, InternKey};
use camino::Utf8PathBuf;
use crate::hir::{typed, type_check::{Env, infer_module, Ty}};

use super::*;

#[salsa::query_group(TyDatabaseStorage)]
pub trait TyDatabase: HirDatabase {
    #[salsa::interned]
    fn intern_ty_path_data(&self, data: TyPathData) -> TyPath;

    fn types(&self, name: Utf8PathBuf) -> typed::Module;

    fn type_at(&self, name: Utf8PathBuf, line: u32, character: u32) -> Option<Ty>;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct TyPath(InternId);

impl TyPath {
    pub fn lookup(&self, db: &impl TyDatabase) -> TyPathData {
        db.lookup_intern_ty_path_data(*self)
    }
}

impl InternKey for TyPath {
    fn from_intern_id(v: InternId) -> Self {
        TyPath(v)
    }

    fn as_intern_id(&self) -> InternId {
        self.0
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TyPathData {
    Root(String),
    Child(TyPath, String),
}

fn types(db: &dyn TyDatabase, name: Utf8PathBuf) -> typed::Module {
    let module = db.hir(name);
    let mut env = Env::empty();
    infer_module(&module, &mut env)
}

fn type_at(db: &dyn TyDatabase, name: Utf8PathBuf, line: u32, character: u32) -> Option<Ty> {
    let module = db.types(name.clone());
    let line_index = db.line_index(name);

    let offset = line_index.find_offset(line, character);

    module.find_ty(offset)
}
