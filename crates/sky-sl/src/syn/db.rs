use super::{parse::ParseResult, cst::LineIndex};
use crate::fs::{FileDatabase, FileId};

#[salsa::query_group(SyntaxDatabaseStorage)]
pub trait SyntaxDatabase: FileDatabase {
    fn get_ast(&self, file: FileId) -> ParseResult;
    fn get_line_index(&self, file: FileId) -> LineIndex;
}

fn get_ast(db: &dyn SyntaxDatabase, file: FileId) -> ParseResult {
    let source = db.file_contents(file);
    let token = crate::lexer::tokenize(&source);
    super::parse::parse(&token, &source)
}

fn get_line_index(db: &dyn SyntaxDatabase, file: FileId) -> LineIndex {
    let source = db.file_contents(file);
    LineIndex::from_str(&source)
}
