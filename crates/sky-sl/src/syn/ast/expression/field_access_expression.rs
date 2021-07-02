use super::super::{AstNode, ExpressionsOwner};
use crate::syn::cst::{SyntaxNode, SyntaxKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FieldAccessExpression {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for FieldAccessExpression {
    fn can_cast_from(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::FieldAccessExpression
    }

    fn cast_from(syntax: SyntaxNode) -> Option<Self>
        where Self: Sized {
        Self::can_cast_from(syntax.kind()).then(|| Self { syntax })
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

impl ExpressionsOwner for FieldAccessExpression {}
