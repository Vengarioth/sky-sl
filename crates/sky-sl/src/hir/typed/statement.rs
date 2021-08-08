use super::ExpressionKind;
use crate::hir::type_check::Ty;
use rowan::{TextRange, TextSize};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum StatementKind {
    Let(LetStatement),
    Expression(ExpressionStatement),
}

impl StatementKind {
    pub fn span(&self) -> TextRange {
        match self {
            StatementKind::Let(s) => s.span,
            StatementKind::Expression(s) => s.span,
        }
    }

    pub fn find_ty(&self, offset: TextSize) -> Option<Ty> {
        match self {
            StatementKind::Let(s) => s.find_ty(offset),
            StatementKind::Expression(s) => s.find_ty(offset),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct LetStatement {
    pub name: String,
    pub expression: ExpressionKind,
    pub ty: Ty,
    pub span: TextRange,
}

impl LetStatement {
    pub fn new(name: String, expression: ExpressionKind, ty: Ty, span: TextRange) -> Self {
        Self {
            name,
            expression,
            ty,
            span,
        }
    }

    pub fn find_ty(&self, offset: TextSize) -> Option<Ty> {
        if self.expression.span().contains(offset) {
            return self.expression.find_ty(offset);
        }

        Some(self.ty)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ExpressionStatement {
    pub expression: ExpressionKind,
    pub ty: Ty,
    pub span: TextRange,
}

impl ExpressionStatement {
    pub fn new(expression: ExpressionKind, ty: Ty, span: TextRange) -> Self {
        Self { expression, ty, span }
    }

    pub fn find_ty(&self, offset: TextSize) -> Option<Ty> {
        if self.expression.span().contains(offset) {
            return self.expression.find_ty(offset);
        }

        None
    }
}
