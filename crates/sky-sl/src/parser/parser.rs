use crate::{lexer::Token, syn::cst::*};
use text_size::TextSize;
use std::sync::Arc;

#[derive(Debug)]
pub struct Parser<'a> {
    events: Vec<ParseEvent>,
    token: &'a [Token],
}

impl<'a> Parser<'a> {
    pub fn new(token: &'a [Token]) -> Self {
        Self {
            events: Vec::new(),
            token,
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
        let length = self.token[0].text_len();
        self.token = &self.token[1..];

        self.events.push(ParseEvent::Token {
            kind,
            length,
        });
    }

    pub fn eof(&self) -> bool {
        self.token.len() == 0
    }

    pub fn begin_node(&mut self) -> Marker {
        let pos = self.events.len() as u32;
        self.events.push(ParseEvent::tombstone());
        Marker::new(pos)
    }

    pub fn emit_error(&mut self) {
        self.events.push(ParseEvent::Error {
            kind: self.current(),
        });
    }

    pub fn finish(self) -> Vec<ParseEvent> {
        self.events
    }

    pub fn process(self) -> GreenNode {

        let mut stack: Vec<(SyntaxKind, Vec<Arc<GreenChild>>)> = Vec::new();
        let mut current: Option<(SyntaxKind, Vec<Arc<GreenChild>>)> = Some((SyntaxKind::Module, Vec::new()));

        for event in self.events {
            match event {
                ParseEvent::BeginNode { kind } => {
                    if let Some(previous) = current.replace((kind, Vec::new())) {
                        stack.push(previous);
                    }
                },
                ParseEvent::EndNode => {
                    let (kind, children) = current.take().unwrap();
                    let node = GreenNode {
                        kind,
                        children,
                    };

                    if let Some((kind, mut previous)) = stack.pop() {
                        previous.push(Arc::new(GreenChild::Node(node)));
                        current = Some((kind, previous));
                    }
                },
                ParseEvent::Token { kind, length } => {
                    if let Some((_, current)) = &mut current {
                        current.push(Arc::new(GreenChild::Token(GreenToken {
                            kind,
                            length,
                        })));
                    }
                },
                _ => (),
            }
        }

        let (kind, children) = current.unwrap();
        let node = GreenNode {
            kind,
            children,
        };

        node
    }
}

#[derive(Debug)]
pub enum ParseEvent {
    BeginNode {
        kind: SyntaxKind,
    },
    EndNode,
    Token {
        kind: SyntaxKind,
        length: TextSize,
    },
    Error {
        kind: SyntaxKind,
    },
}

impl ParseEvent {
    pub fn tombstone() -> Self {
        ParseEvent::BeginNode {
            kind: SyntaxKind::Error,
        }
    }
}

#[must_use]
#[derive(Debug)]
pub struct Marker {
    pos: u32,
}

impl Marker {
    fn new(pos: u32) -> Self {
        Self {
            pos,
        }
    }

    pub fn complete(self, parser: &mut Parser, kind: SyntaxKind) {
        let index = self.pos as usize;
        match parser.events[index] {
            ParseEvent::BeginNode { kind: ref mut slot, .. } => *slot = kind,
            _ => unreachable!()
        }
        parser.events.push(ParseEvent::EndNode);
    }
}
