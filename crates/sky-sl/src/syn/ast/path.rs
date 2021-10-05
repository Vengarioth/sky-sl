use super::{AstNode, NameOwner};
use crate::syn::cst::{SyntaxKind, SyntaxNode};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Path {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for Path {
    fn can_cast_from(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::Path
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

pub trait PathOwner: AstNode {
    fn path(&self) -> Option<Path> {
        super::child(self)
    }
}

impl PathSegmentOwner for Path {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PathSegment {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for PathSegment {
    fn can_cast_from(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PathSegment
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

pub trait PathSegmentOwner: AstNode {
    fn segment(&self) -> Option<PathSegment> {
        super::child(self)
    }
}

impl NameOwner for PathSegment {}
impl PathSegmentOwner for PathSegment {}
