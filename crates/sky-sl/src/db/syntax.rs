use crate::syn::{cst::*, ast::*};
use camino::Utf8PathBuf;
use std::sync::Arc;

use super::*;

#[salsa::query_group(SyntaxDatabaseStorage)]
pub trait SyntaxDatabase: SourceDatabase {
    fn cst(&self, name: Utf8PathBuf) -> SyntaxNode;
    fn ast(&self, name: Utf8PathBuf) -> Arc<Root>;
}

fn cst(db: &dyn SyntaxDatabase, name: Utf8PathBuf) -> SyntaxNode {
    let input = db.input_file(name);
    let token = crate::lexer::tokenize(&input);
    let result = crate::parser::parse(&token, &input);
    result.root
}

fn ast(db: &dyn SyntaxDatabase, name: Utf8PathBuf) -> Arc<Root> {
    let syntax = db.cst(name);

    Arc::new(Root {
        syntax,
    })
}
