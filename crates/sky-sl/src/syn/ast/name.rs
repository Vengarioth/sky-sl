use super::{AstNode, IdentifierOwner};
use crate::syn::cst::{SyntaxKind, SyntaxNode};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Name {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for Name {
    fn can_cast_from(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::Name
    }

    fn cast_from(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        Self::can_cast_from(syntax.kind()).then(|| Self { syntax })
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

pub trait NameOwner: AstNode {
    fn name(&self) -> Option<Name> {
        super::child(self)
    }
}

impl IdentifierOwner for Name {}
