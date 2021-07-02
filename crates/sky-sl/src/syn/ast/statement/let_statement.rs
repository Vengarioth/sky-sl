use super::super::{AstNode, AstChildren, ExpressionOwner, IdentifierOwner};
use crate::syn::cst::{SyntaxNode, SyntaxKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LetStatement {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for LetStatement {
    fn can_cast_from(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::LetStatement
    }

    fn cast_from(syntax: SyntaxNode) -> Option<Self>
        where Self: Sized {
        Self::can_cast_from(syntax.kind()).then(|| Self { syntax })
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

pub trait LetStatementsOwner: AstNode {
    fn let_statements(&self) -> AstChildren<LetStatement> {
        super::super::children(self)
    }
}

impl ExpressionOwner for LetStatement {}
impl IdentifierOwner for LetStatement {}
