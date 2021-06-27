use crate::{lexer::Token, syn::cst::*, syn::Parse, syn::ast::Root};
use super::{SyntaxError, ErrorKind, ParseError};

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

    pub fn is_at(&self, kind: SyntaxKind) -> Result<bool, ParseError> {
        let current = self.current()?;
        Ok(current == kind)
    }

    pub fn current(&self) -> Result<SyntaxKind, ParseError> {
        if self.eof() {
            return Err(ParseError::EOF);
        }

        Ok(self.token[0].kind())
    }

    /// Shorthand for parsing whitespace
    pub fn ws(&mut self) -> &mut Self {
        if !self.eof() && self.is_at(SyntaxKind::Whitespace).unwrap() {
            self.bump().unwrap();
        }

        self
    }

    pub fn bump_if(&mut self, kind: SyntaxKind) -> Result<(), ParseError> {
        if self.is_at(kind)? {
            self.bump()?;
        }

        Ok(())
    }

    pub fn bump(&mut self) -> Result<(), ParseError> {
        if self.eof() {
            return Err(ParseError::EOF);
        }

        let kind = self.token[0].kind();
        let length = self.token[0].len();

        let current_str = &self.input[0..length];
        self.input = &self.input[length..];
        self.token = &self.token[1..];
        self.offset += length;

        self.emit_token(kind, current_str);
        Ok(())
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

    pub fn checkpoint(&mut self) -> Checkpoint {
        self.builder.checkpoint()
    }

    pub fn begin_node_at(&mut self, checkpoint: Checkpoint, kind: SyntaxKind) {
        self.builder.start_node_at(checkpoint, kind.into());
    }

    pub fn node(&mut self, kind: SyntaxKind, f: impl Fn(&mut Self) -> Result<(), ParseError>) -> Result<(), ParseError> {
        self.begin_node(kind);

        let r = f(self);

        self.end_node();

        r
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
}
