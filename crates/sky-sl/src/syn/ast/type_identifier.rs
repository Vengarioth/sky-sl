use super::AstNode;
use crate::syn::cst::{SyntaxNode, SyntaxKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TypeIdentifier {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for TypeIdentifier {
    fn can_cast_from(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::TypeIdentifier
    }

    fn cast_from(syntax: SyntaxNode) -> Option<Self>
        where Self: Sized {
        Self::can_cast_from(syntax.kind()).then(|| Self { syntax })
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

pub trait TypeIdentifierOwner: AstNode {
    fn type_identifier(&self) -> Option<TypeIdentifier> {
        super::child(self)
    }
}
