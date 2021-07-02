use super::super::{AstNode, IdentifierOwner, ExpressionOwner, AstChildren};
use crate::syn::cst::{SyntaxNode, SyntaxKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StructExpression {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for StructExpression {
    fn can_cast_from(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::StructExpression
    }

    fn cast_from(syntax: SyntaxNode) -> Option<Self>
        where Self: Sized {
        Self::can_cast_from(syntax.kind()).then(|| Self { syntax })
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

impl ExpressionOwner for StructExpression {}

impl StructExpressionFieldsOwner for StructExpression {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StructExpressionFields {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for StructExpressionFields {
    fn can_cast_from(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::StructExpressionFields
    }

    fn cast_from(syntax: SyntaxNode) -> Option<Self>
        where Self: Sized {
        Self::can_cast_from(syntax.kind()).then(|| Self { syntax })
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

impl StructExpressionFieldOwner for StructExpressionFields {}

pub trait StructExpressionFieldsOwner: AstNode {
    fn fields(&self) -> Option<StructExpressionFields> {
        super::super::child(self)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StructExpressionField {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for StructExpressionField {
    fn can_cast_from(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::StructExpressionField
    }

    fn cast_from(syntax: SyntaxNode) -> Option<Self>
        where Self: Sized {
        Self::can_cast_from(syntax.kind()).then(|| Self { syntax })
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

impl IdentifierOwner for StructExpressionField {}
impl ExpressionOwner for StructExpressionField {}

pub trait StructExpressionFieldOwner: AstNode {
    fn fields(&self) -> AstChildren<StructExpressionField> {
        super::super::children(self)
    }
}
