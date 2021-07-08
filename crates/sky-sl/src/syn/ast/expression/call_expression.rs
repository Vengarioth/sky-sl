use super::super::{AstNode, ExpressionOwner, ExpressionsOwner};
use crate::syn::cst::{SyntaxNode, SyntaxKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CallExpression {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for CallExpression {
    fn can_cast_from(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CallExpression
    }

    fn cast_from(syntax: SyntaxNode) -> Option<Self>
        where Self: Sized {
        Self::can_cast_from(syntax.kind()).then(|| Self { syntax })
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

impl ExpressionOwner for CallExpression {}
impl CallArgumentListOwner for CallExpression {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CallArgumentList {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for CallArgumentList {
    fn can_cast_from(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CallArgumentList
    }

    fn cast_from(syntax: SyntaxNode) -> Option<Self>
        where Self: Sized {
        Self::can_cast_from(syntax.kind()).then(|| Self { syntax })
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

impl ExpressionsOwner for CallArgumentList {}

pub trait CallArgumentListOwner: AstNode {
    fn arguments(&self) -> Option<CallArgumentList> {
        super::super::child(self)
    }
}
