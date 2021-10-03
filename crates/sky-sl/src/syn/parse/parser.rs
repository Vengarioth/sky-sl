use super::ParseDiagnostic;
use crate::{
    lexer::Token,
    syn::{
        ast::{AstNode, Root},
        cst::*,
    },
};

#[derive(Debug)]
pub struct Parser<'a> {
    builder: Builder<'a>,
    token: &'a [Token],
    input: &'a str,
    diagnostics: Vec<ParseDiagnostic>,
    offset: usize,
}

impl<'a> Parser<'a> {
    pub fn new(token: &'a [Token], input: &'a str) -> Self {
        Self {
            builder: Builder::new(),
            input,
            token,
            diagnostics: Vec::new(),
            offset: 0,
        }
    }

    pub fn is_at(&self, kind: SyntaxKind) -> bool {
        self.current() == kind
    }

    pub fn is_at_any(&self, token: &[SyntaxKind]) -> bool {
        token.iter().any(|kind| self.is_at(*kind))
    }

    pub fn current(&self) -> SyntaxKind {
        if self.eof() {
            return SyntaxKind::EOF;
        }

        self.token[0].kind()
    }

    pub fn next(&self) -> Option<SyntaxKind> {
        if self.token.len() < 2 {
            return None;
        }

        Some(self.token[1].kind())
    }

    /// consumes zero or one whitespace token
    pub fn ws0(&mut self) {
        if self.is_at(SyntaxKind::Whitespace) {
            self.bump();
        }
    }

    /// consumes one whitespace token
    pub fn ws1(&mut self) {
        if self.is_at(SyntaxKind::Whitespace) {
            self.bump();
        } else {
            self.diagnostics.push(ParseDiagnostic::MissingToken {
                location: (self.offset as u32).into(),
                expected: vec![SyntaxKind::Whitespace],
            });
        }
    }

    pub fn consume_if(&mut self, kind: SyntaxKind) -> bool {
        if self.is_at(kind) {
            self.consume(kind);
            true
        } else {
            false
        }
    }

    pub fn consume_if_any(&mut self, token: &[SyntaxKind]) -> Option<SyntaxKind> {
        if self.is_at_any(token) {
            let current = self.current();
            self.bump();
            Some(current)
        } else {
            None
        }
    }

    pub fn consume_any(&mut self, token: &[SyntaxKind]) -> SyntaxKind {
        if self.is_at_any(token) {
            let current = self.current();
            self.bump();
            current
        } else {
            panic!("called consume_any on the wrong token");
        }
    }

    pub fn consume(&mut self, kind: SyntaxKind) {
        if !self.is_at(kind) {
            panic!("called consume on the wrong token");
        }

        self.bump();
    }

    pub fn expect(&mut self, token: SyntaxKind, recover: &[SyntaxKind]) {
        loop {
            if !self.is_at(token) {
                if self.is_at_any(recover) {
                    self.missing(&[token]);
                    break;
                } else if self.eof() {
                    self.missing(&[token]);
                    break;
                } else {
                    self.skip(&[token]);
                }
            } else {
                self.consume(token);
                break;
            }
        }
    }

    pub fn expect_any(
        &mut self,
        token: &[SyntaxKind],
        recover: &[SyntaxKind],
    ) -> Option<SyntaxKind> {
        loop {
            if !self.is_at_any(token) {
                if self.is_at_any(recover) {
                    self.missing(token);
                    break;
                } else if self.eof() {
                    self.missing(token);
                    break;
                } else {
                    self.skip(token);
                }
            } else {
                let current = self.current();
                self.bump();
                return Some(current);
            }
        }

        None
    }

    fn bump(&mut self) {
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

    pub fn missing(&mut self, expected: &[SyntaxKind]) {
        self.diagnostics.push(ParseDiagnostic::MissingToken {
            location: (self.offset as u32).into(),
            expected: expected.into(),
        });
    }

    pub fn skip(&mut self, expected: &[SyntaxKind]) {
        let skipped = self.current();
        self.begin_node(SyntaxKind::Error);
        self.bump();
        self.end_node();
        self.diagnostics.push(ParseDiagnostic::SkippedToken {
            location: (self.offset as u32).into(),
            skipped,
            expected: expected.into(),
        });
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

    pub fn finish(self) -> ParseResult {
        let root = self.builder.finish();
        let diagnostics = self.diagnostics;

        ParseResult::new(root, diagnostics)
    }

    pub fn remaining(&self) -> Vec<SyntaxKind> {
        self.token.iter().map(|t| t.kind()).collect()
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ParseResult {
    pub root: GreenNode,
    pub diagnostics: Vec<ParseDiagnostic>,
}

impl ParseResult {
    pub fn new(root: GreenNode, diagnostics: Vec<ParseDiagnostic>) -> Self {
        Self { root, diagnostics }
    }

    pub fn diagnostics(&self) -> &[ParseDiagnostic] {
        &self.diagnostics
    }

    pub fn tree(&self) -> Root {
        <Root as AstNode>::cast_from(SyntaxNode::new_root(self.root.clone())).unwrap()
    }
}
