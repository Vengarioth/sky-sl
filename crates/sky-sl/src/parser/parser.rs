use crate::{lexer::Token, syn::cst::*, syn::Parse, syn::ast::Root};
use super::{ParseError, ErrorKind};

#[derive(Debug)]
pub struct Parser<'a> {
    builder: Builder<'a>,
    token: &'a [Token],
    input: &'a str,
    errors: Vec<ParseError>,
    offset: usize,
}

impl<'a> Parser<'a> {
    pub fn new(token: &'a [Token], input: &'a str) -> Self {
        Self {
            builder: Builder::new(),
            input,
            token,
            errors: Vec::new(),
            offset: 0,
        }
    }

    pub fn is_at(&self, kind: SyntaxKind) -> bool {
        self.current() == kind
    }

    pub fn current(&self) -> SyntaxKind {
        self.token[0].kind()
    }

    pub fn bump_if(&mut self, kind: SyntaxKind) {
        if self.is_at(kind) {
            self.bump();
        }
    }

    pub fn bump(&mut self) {
        let kind = self.token[0].kind();
        let length = self.token[0].len();

        let current_str = &self.input[0..length];
        self.input = &self.input[length..];
        self.token = &self.token[1..];
        self.offset += length;

        self.emit_token(kind, current_str);
    }

    pub fn recover(&mut self, _recover_points: &[SyntaxKind]) {
    }

    pub fn eof(&self) -> bool {
        self.token.len() == 0
    }

    pub fn begin_node(&mut self, kind: SyntaxKind) {
        self.builder.start_node(kind.into());
    }

    pub fn end_node(&mut self) {
        self.builder.finish_node();
    }

    pub fn emit_token(&mut self, kind: SyntaxKind, text: &str) {
        self.builder.token(kind.into(), text);
    }

    pub fn emit_error(&mut self, kind: ErrorKind) {
        self.errors.push(ParseError {
            offset: self.offset,
            kind,
        });
    }

    pub fn finish(self) -> ParseResult {
        let root = Parse::new(self.builder.finish());
        let errors = self.errors;

        ParseResult {
            root,
            errors,
        }
    }
}

#[derive(Debug)]
pub struct ParseResult {
    pub root: Parse<Root>,
    pub errors: Vec<ParseError>,
}
