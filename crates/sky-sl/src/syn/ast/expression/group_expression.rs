use super::AstNode;
use crate::syn::cst::{SyntaxNode, SyntaxKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GroupExpression {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for GroupExpression {
    fn can_cast_from(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::GroupExpression
    }

    fn cast_from(syntax: SyntaxNode) -> Option<Self>
        where Self: Sized {
        Self::can_cast_from(syntax.kind()).then(|| Self { syntax })
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
