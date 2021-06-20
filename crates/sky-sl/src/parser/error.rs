use crate::syn::cst::SyntaxKind;
use thiserror::*;

#[derive(Debug)]
pub enum ErrorKind {
    NotFound(SyntaxKind),
    Unexpected(SyntaxKind),
}

#[derive(Debug)]
pub struct SyntaxError {
    pub offset: usize,
    pub kind: ErrorKind,
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Unexpected end of file")]
    EOF,
}
