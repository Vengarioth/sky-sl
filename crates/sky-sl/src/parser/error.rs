use crate::syn::cst::SyntaxKind;

#[derive(Debug)]
pub enum ErrorKind {
    NotFound(SyntaxKind),
    Unexpected(SyntaxKind),
}

#[derive(Debug)]
pub struct ParseError {
    pub offset: usize,
    pub kind: ErrorKind,
}
