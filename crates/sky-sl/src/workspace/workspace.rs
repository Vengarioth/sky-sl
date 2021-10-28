use super::{db::CompilerDatabase, CompileError};
use crate::fs::{initialize_fs, insert_file, lookup_file, remove_file, FileId, FileSystemError};
use crate::intern::{InternDatabase, Name};
use crate::package::{Package, PackageDatabase};
use crate::syn::cst::LineIndex;
use crate::syn::db::SyntaxDatabase;
use crate::hir;
use crate::hir::*;
use crate::syn::parse::ParseResult;
use camino::{Utf8Path, Utf8PathBuf};
use std::sync::Arc;

#[derive(Debug)]
pub struct Workspace {
    root_path: Utf8PathBuf,
    db: CompilerDatabase,
}

impl Workspace {
    pub fn create(root_path: Utf8PathBuf) -> Self {
        let mut db = CompilerDatabase::default();
        initialize_fs(&mut db);

        Self { root_path, db }
    }

    pub fn insert_file(
        &mut self,
        path: &Utf8Path,
        contents: Arc<String>,
    ) -> Result<FileId, FileSystemError> {
        let path = path.strip_prefix(&self.root_path).unwrap();
        insert_file(&mut self.db, path, contents)
    }

    pub fn remove_file(&mut self, path: &Utf8Path) -> Result<(), FileSystemError> {
        let path = path.strip_prefix(&self.root_path).unwrap();
        remove_file(&mut self.db, path)
    }

    pub fn find_packages(&self) -> Vec<Package> {
        self.db.find_packages()
    }

    pub fn get_ast(&mut self, path: &Utf8Path) -> Result<ParseResult, CompileError> {
        let path = path.strip_prefix(&self.root_path).unwrap();
        let file_id = lookup_file(&self.db, path).ok_or_else(|| {
            CompileError::FileSystemError(FileSystemError::FileDoesNotExist(path.to_owned()))
        })?;

        let ast = self.db.get_ast(file_id);

        Ok(ast)
    }

    pub fn get_line_index(&mut self, path: &Utf8Path) -> Result<LineIndex, CompileError> {
        let path = path.strip_prefix(&self.root_path).unwrap();
        let file_id = lookup_file(&self.db, path).ok_or_else(|| {
            CompileError::FileSystemError(FileSystemError::FileDoesNotExist(path.to_owned()))
        })?;
        let line_index = self.db.get_line_index(file_id);
        Ok(line_index)
    }

    pub fn get_symbols(&mut self, path: &Utf8Path) -> Result<hir::symbol::SymbolList, CompileError> {
        let path = path.strip_prefix(&self.root_path).unwrap();
        let file_id = lookup_file(&self.db, path).ok_or_else(|| {
            CompileError::FileSystemError(FileSystemError::FileDoesNotExist(path.to_owned()))
        })?;
        let symbols = self.db.get_symbols(file_id);
        Ok(symbols)
    }

    pub fn get_hir(&mut self, path: &Utf8Path) -> Result<hir::untyped::Module, CompileError> {
        let path = path.strip_prefix(&self.root_path).unwrap();
        let file_id = lookup_file(&self.db, path).ok_or_else(|| {
            CompileError::FileSystemError(FileSystemError::FileDoesNotExist(path.to_owned()))
        })?;
        let hir = self.db.get_hir(file_id);
        Ok(hir)
    }

    pub fn get_typed_hir(&mut self, path: &Utf8Path) -> Result<hir::typed::Module, CompileError> {
        let path = path.strip_prefix(&self.root_path).unwrap();
        let file_id = lookup_file(&self.db, path).ok_or_else(|| {
            CompileError::FileSystemError(FileSystemError::FileDoesNotExist(path.to_owned()))
        })?;
        let typed_hir = self.db.get_typed_hir(file_id);
        Ok(typed_hir)
    }

    pub fn get_item_at(&mut self, path: &Utf8Path, _line: u32, _character: u32) -> Result<(), CompileError> {
        let path = path.strip_prefix(&self.root_path).unwrap();

        let file_id = lookup_file(&self.db, path).ok_or_else(|| {
            CompileError::FileSystemError(FileSystemError::FileDoesNotExist(path.to_owned()))
        })?;

        let _hir = self.db.get_hir(file_id);

        Ok(())
    }

    pub fn interned_name(&self, name: Name) -> String {
        self.db.lookup_intern_name(name)
    }
}
