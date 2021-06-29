use crate::syn::cst::*;
use super::{AstNode, AstChildren, IdentifierOwner, TypeIdentifierOwner};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ArgumentList {
    syntax: SyntaxNode,
}

impl AstNode for ArgumentList {
    fn can_cast_from(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ArgumentList
    }

    fn cast_from(syntax: SyntaxNode) -> Option<Self> where Self: Sized {
        Self::can_cast_from(syntax.kind()).then(|| Self { syntax })
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

pub trait ArgumentListOwner: AstNode {
    fn argument_list(&self) -> Option<ArgumentList> {
        super::child(self)
    }
}

impl ArgumentsOwner for ArgumentList {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Argument {
    syntax: SyntaxNode,
}

impl AstNode for Argument {
    fn can_cast_from(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::Argument
    }

    fn cast_from(syntax: SyntaxNode) -> Option<Self> where Self: Sized {
        Self::can_cast_from(syntax.kind()).then(|| Self { syntax })
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

impl IdentifierOwner for Argument {}
impl TypeIdentifierOwner for Argument {}

pub trait ArgumentsOwner: AstNode {
    fn arguments(&self) -> AstChildren<Argument> {
        super::children(self)
    }
}
