use crate::syn::{Parse, ast::Root};
use camino::Utf8PathBuf;

use super::*;

#[salsa::query_group(SyntaxDatabaseStorage)]
pub trait SyntaxDatabase: SourceDatabase {
    fn ast(&self, name: Utf8PathBuf) -> Parse<Root>;
}

fn ast(db: &dyn SyntaxDatabase, name: Utf8PathBuf) -> Parse<Root> {
    let input = db.input_file(name);
    let token = crate::lexer::tokenize(&input);
    crate::parser::parse(&token, &input)
}
