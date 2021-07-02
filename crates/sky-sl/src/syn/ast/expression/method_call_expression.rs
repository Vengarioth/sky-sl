use super::super::{AstNode};
use crate::syn::cst::{SyntaxNode, SyntaxKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MethodCallExpression {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for MethodCallExpression {
    fn can_cast_from(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::MethodCallExpression
    }

    fn cast_from(syntax: SyntaxNode) -> Option<Self>
        where Self: Sized {
        Self::can_cast_from(syntax.kind()).then(|| Self { syntax })
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
