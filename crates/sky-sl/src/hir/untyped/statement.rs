use crate::intern::Name;

use super::ExpressionKind;
use rowan::TextRange;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum StatementKind {
    Let(LetStatement),
    Expression(ExpressionStatement),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct LetStatement {
    pub name: Name,
    pub expression: ExpressionKind,
    pub span: TextRange,
}

impl LetStatement {
    pub fn new(name: Name, expression: ExpressionKind, span: TextRange) -> Self {
        Self {
            name,
            expression,
            span,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ExpressionStatement {
    pub expression: ExpressionKind,
    pub span: TextRange,
}

impl ExpressionStatement {
    pub fn new(expression: ExpressionKind, span: TextRange) -> Self {
        Self { expression, span }
    }
}
