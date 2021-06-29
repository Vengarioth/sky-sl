use super::AstNode;
use crate::syn::cst::{SyntaxNode, SyntaxKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BinaryExpression {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for BinaryExpression {
    fn can_cast_from(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::BinaryExpression
    }

    fn cast_from(syntax: SyntaxNode) -> Option<Self>
        where Self: Sized {
        Self::can_cast_from(syntax.kind()).then(|| Self { syntax })
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
