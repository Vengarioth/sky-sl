use crate::syn::cst::SyntaxKind;
use thiserror::*;

#[derive(Debug, Eq, PartialEq)]
pub enum ErrorKind {
    NotFound(SyntaxKind),
    Unexpected(SyntaxKind),
}

#[derive(Debug, Eq, PartialEq)]
pub struct SyntaxError {
    pub offset: usize,
    pub length: usize,
    pub kind: ErrorKind,
}
