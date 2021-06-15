use super::AstNode;
use crate::syn::cst::{SyntaxNode, SyntaxKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Identifier {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for Identifier {
    fn can_cast_from(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::Identifier
    }

    fn cast_from(syntax: SyntaxNode) -> Option<Self>
        where Self: Sized {
        Self::can_cast_from(syntax.kind()).then(|| Self { syntax })
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

pub trait IdentifierOwner: AstNode {
    fn identifier(&self) -> Option<Identifier> {
        super::child(self)
    }
}
