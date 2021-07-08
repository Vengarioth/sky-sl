use crate::syn::cst::*;
use super::{AstNode, TypeIdentifierOwner};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ReturnType {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for ReturnType {
    fn can_cast_from(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ReturnType
    }

    fn cast_from(syntax: SyntaxNode) -> Option<Self>
        where Self: Sized {
        Self::can_cast_from(syntax.kind()).then(|| Self { syntax })
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

pub trait ReturnTypeOwner: AstNode {
    fn return_type(&self) -> Option<ReturnType> {
        super::child(self)
    }
}

impl TypeIdentifierOwner for ReturnType {}
