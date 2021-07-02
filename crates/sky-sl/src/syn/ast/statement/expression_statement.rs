use super::super::{AstNode, AstChildren, ExpressionOwner};
use crate::syn::cst::{SyntaxNode, SyntaxKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ExpressionStatement {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for ExpressionStatement {
    fn can_cast_from(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ExpressionStatement
    }

    fn cast_from(syntax: SyntaxNode) -> Option<Self>
        where Self: Sized {
        Self::can_cast_from(syntax.kind()).then(|| Self { syntax })
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

pub trait ExpressionStatementsOwner: AstNode {
    fn let_statements(&self) -> AstChildren<ExpressionStatement> {
        super::super::children(self)
    }
}

impl ExpressionOwner for ExpressionStatement {}
