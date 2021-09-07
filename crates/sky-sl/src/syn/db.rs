use super::{Parse, ast::Root, cst::LineIndex};
use crate::fs::{FileDatabase, FileId};

#[salsa::query_group(SyntaxDatabaseStorage)]
pub trait SyntaxDatabase: FileDatabase {
    fn get_ast(&self, file: FileId) -> Parse<Root>;
    fn get_line_index(&self, file: FileId) -> LineIndex;
}

fn get_ast(db: &dyn SyntaxDatabase, file: FileId) -> Parse<Root> {
    let source = db.file_contents(file);
    let token = crate::lexer::tokenize(&source);
    crate::parser::parse(&token, &source)
}

fn get_line_index(db: &dyn SyntaxDatabase, file: FileId) -> LineIndex {
    let source = db.file_contents(file);
    LineIndex::from_str(&source)
}
