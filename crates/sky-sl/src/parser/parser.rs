use crate::{lexer::Token, syn::cst::*, syn::Parse, syn::ast::Root};
use super::{ErrorKind, SyntaxError, TokenSet};

#[derive(Debug)]
pub struct Parser<'a> {
    builder: Builder<'a>,
    token: &'a [Token],
    input: &'a str,
    errors: Vec<SyntaxError>,
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

    pub fn is_at_set(&self, set: &TokenSet) -> bool {
        set.contains(&self.current())
    }

    pub fn current(&self) -> SyntaxKind {
        if self.eof() {
            return SyntaxKind::EOF;
        }

        self.token[0].kind()
    }

    pub fn current_char(&self) -> Option<char> {
        self.input.chars().nth(0)
    }

    pub fn next(&self) -> Option<SyntaxKind> {
        if self.token.len() < 2 {
            return None;
        }

        Some(self.token[1].kind())
    }

    /// Shorthand for parsing whitespace
    pub fn ws(&mut self) -> &mut Self {
        if self.is_at(SyntaxKind::Whitespace) {
            self.bump();
        }

        self
    }

    pub fn bump_if(&mut self, kind: SyntaxKind) {
        if self.is_at(kind) {
            self.bump();
        }
    }

    pub fn bump(&mut self) {
        if self.eof() {
            return;
        }

        let kind = self.token[0].kind();
        let length = self.token[0].len();

        let current_str = &self.input[0..length];
        self.input = &self.input[length..];
        self.token = &self.token[1..];
        self.offset += length;

        self.emit_token(kind, current_str);
    }

    pub fn error_and_recover(&mut self, error: ErrorKind, kinds: &TokenSet) {
        if self.is_at(SyntaxKind::OpenBrace) || self.is_at(SyntaxKind::CloseBrace) || self.is_at_set(kinds) {
            self.emit_error(error);
        } else {
            self.begin_node(SyntaxKind::Error);
            self.emit_error(error);
            self.bump();
            self.end_node();
        }
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

    pub fn checkpoint(&mut self) -> Checkpoint {
        self.builder.checkpoint()
    }

    pub fn begin_node_at(&mut self, checkpoint: Checkpoint, kind: SyntaxKind) {
        self.builder.start_node_at(checkpoint, kind.into());
    }

    pub fn node_at(&mut self, checkpoint: Checkpoint, kind: SyntaxKind, f: impl Fn(&mut Self)) {
        self.begin_node_at(checkpoint, kind);
        f(self);
        self.end_node();
    }

    pub fn node(&mut self, kind: SyntaxKind, f: impl Fn(&mut Self)) {
        self.begin_node(kind);
        f(self);
        self.end_node();
    }

    pub fn emit_token(&mut self, kind: SyntaxKind, text: &str) {
        self.builder.token(kind.into(), text);
    }

    pub fn emit_error(&mut self, kind: ErrorKind) {
        self.errors.push(SyntaxError {
            offset: self.offset,
            length: self.token.first().map(|t| t.len()).unwrap_or(1),
            kind,
        });
    }

    pub fn finish(self) -> Parse<Root> {
        let root = self.builder.finish();
        Parse::new(root, self.errors)
    }

    pub fn remaining(&self) -> Vec<SyntaxKind> {
        self.token.iter().map(|t| t.kind()).collect()
    }
}
