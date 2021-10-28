use super::{AstNode, AstChildren};
use crate::syn::cst::{SyntaxNode, SyntaxKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UseDeclaration {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for UseDeclaration {
    fn can_cast_from(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::UseDeclaration
    }

    fn cast_from(syntax: SyntaxNode) -> Option<Self>
        where Self: Sized {
        Self::can_cast_from(syntax.kind()).then(|| Self { syntax })
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

impl UseTreeOwner for UseDeclaration {}

pub trait UseDeclarationOwner: AstNode {
    fn uses(&self) -> AstChildren<UseDeclaration> {
        super::children(self)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UseTree {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for UseTree {
    fn can_cast_from(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::UseTree
    }

    fn cast_from(syntax: SyntaxNode) -> Option<Self>
        where Self: Sized {
        Self::can_cast_from(syntax.kind()).then(|| Self { syntax })
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

pub trait UseTreeOwner: AstNode {
    fn use_tree(&self) -> Option<UseTree> {
        super::child(self)
    }
}
