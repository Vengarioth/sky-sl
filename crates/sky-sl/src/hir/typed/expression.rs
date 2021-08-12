use crate::hir::type_check::Ty;
use rowan::{TextRange, TextSize};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ExpressionKind {
    LiteralExpression(LiteralExpression),
    BinaryExpression(BinaryExpression),
    GroupExpression(GroupExpression),
    CallExpression(CallExpression),
    FieldAccessExpression(FieldAccessExpression),
    IndexExpression(IndexExpression),
    PathExpression(PathExpression),
    StructExpression(StructExpression),
}

impl ExpressionKind {
    pub fn span(&self) -> TextRange {
        match self {
            ExpressionKind::LiteralExpression(e) => e.span,
            ExpressionKind::BinaryExpression(e) => e.span,
            ExpressionKind::GroupExpression(e) => e.span,
            ExpressionKind::CallExpression(e) => e.span,
            ExpressionKind::FieldAccessExpression(e) => e.span,
            ExpressionKind::IndexExpression(e) => e.span,
            ExpressionKind::PathExpression(e) => e.span,
            ExpressionKind::StructExpression(e) => e.span,
        }
    }

    pub fn ty(&self) -> Ty {
        match self {
            ExpressionKind::LiteralExpression(e) => e.ty,
            ExpressionKind::BinaryExpression(e) => e.ty,
            ExpressionKind::GroupExpression(e) => e.ty,
            ExpressionKind::CallExpression(e) => e.ty,
            ExpressionKind::FieldAccessExpression(e) => e.ty,
            ExpressionKind::IndexExpression(e) => e.ty,
            ExpressionKind::PathExpression(e) => e.ty,
            ExpressionKind::StructExpression(e) => e.ty,
        }
    }

    pub fn find_ty(&self, offset: TextSize) -> Option<Ty> {
        match self {
            ExpressionKind::LiteralExpression(e) => e.find_ty(offset),
            ExpressionKind::BinaryExpression(e) => e.find_ty(offset),
            ExpressionKind::GroupExpression(e) => e.find_ty(offset),
            ExpressionKind::CallExpression(e) => e.find_ty(offset),
            ExpressionKind::FieldAccessExpression(e) => e.find_ty(offset),
            ExpressionKind::IndexExpression(e) => e.find_ty(offset),
            ExpressionKind::PathExpression(e) => e.find_ty(offset),
            ExpressionKind::StructExpression(e) => e.find_ty(offset),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct LiteralExpression {
    pub ty: Ty,
    pub span: TextRange,
}

impl LiteralExpression {
    pub fn new(ty: Ty, span: TextRange) -> Self {
        Self { ty, span }
    }

    pub fn find_ty(&self, _offset: TextSize) -> Option<Ty> {
        Some(self.ty)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct GroupExpression {
    pub inner: Box<ExpressionKind>,
    pub ty: Ty,
    pub span: TextRange,
}

impl GroupExpression {
    pub fn new(inner: Box<ExpressionKind>, ty: Ty, span: TextRange) -> Self {
        Self { inner, ty, span }
    }

    pub fn find_ty(&self, offset: TextSize) -> Option<Ty> {
        if self.inner.span().contains(offset) {
            return self.inner.find_ty(offset);
        }

        Some(self.ty)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct BinaryExpression {
    pub lhs: Box<ExpressionKind>,
    pub rhs: Box<ExpressionKind>,
    pub ty: Ty,
    pub span: TextRange,
}

impl BinaryExpression {
    pub fn new(
        lhs: Box<ExpressionKind>,
        rhs: Box<ExpressionKind>,
        ty: Ty,
        span: TextRange,
    ) -> Self {
        Self { lhs, rhs, ty, span }
    }

    pub fn find_ty(&self, offset: TextSize) -> Option<Ty> {
        if self.lhs.span().contains(offset) {
            return self.lhs.find_ty(offset);
        }

        if self.rhs.span().contains(offset) {
            return self.rhs.find_ty(offset);
        }

        Some(self.ty)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct CallExpression {
    pub arguments: Vec<ExpressionKind>,
    pub ty: Ty,
    pub span: TextRange,
}

impl CallExpression {
    pub fn new(arguments: Vec<ExpressionKind>, ty: Ty, span: TextRange) -> Self {
        Self {
            arguments,
            ty,
            span,
        }
    }

    pub fn find_ty(&self, offset: TextSize) -> Option<Ty> {
        for argument in &self.arguments {
            if argument.span().contains(offset) {
                return argument.find_ty(offset);
            }
        }

        Some(self.ty)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FieldAccessExpression {
    pub ty: Ty,
    pub span: TextRange,
}

impl FieldAccessExpression {
    pub fn new(ty: Ty, span: TextRange) -> Self {
        Self { ty, span }
    }

    pub fn find_ty(&self, _offset: TextSize) -> Option<Ty> {
        Some(self.ty)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct IndexExpression {
    pub ty: Ty,
    pub span: TextRange,
}

impl IndexExpression {
    pub fn new(ty: Ty, span: TextRange) -> Self {
        Self { ty, span }
    }

    pub fn find_ty(&self, _offset: TextSize) -> Option<Ty> {
        todo!()
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PathExpression {
    pub path: String,
    pub ty: Ty,
    pub span: TextRange,
}

impl PathExpression {
    pub fn new(path: String, ty: Ty, span: TextRange) -> Self {
        Self { path, ty, span }
    }

    pub fn find_ty(&self, _offset: TextSize) -> Option<Ty> {
        Some(self.ty)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct StructExpression {
    pub ty: Ty,
    pub span: TextRange,
}

impl StructExpression {
    pub fn new(ty: Ty, span: TextRange) -> Self {
        Self { ty, span }
    }

    pub fn find_ty(&self, _offset: TextSize) -> Option<Ty> {
        todo!()
    }
}
