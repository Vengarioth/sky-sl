use crate::syn::cst::*;
use super::{AstNode, AstChildren, IdentifierOwner, TypeIdentifierOwner};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MemberList {
    syntax: SyntaxNode,
}

impl AstNode for MemberList {
    fn can_cast_from(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::MemberList
    }

    fn cast_from(syntax: SyntaxNode) -> Option<Self> where Self: Sized {
        Self::can_cast_from(syntax.kind()).then(|| Self { syntax })
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

pub trait MemberListOwner: AstNode {
    fn member_list(&self) -> Option<MemberList> {
        super::child(self)
    }
}

impl MemberOwner for MemberList {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Member {
    syntax: SyntaxNode,
}

impl AstNode for Member {
    fn can_cast_from(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::Member
    }

    fn cast_from(syntax: SyntaxNode) -> Option<Self> where Self: Sized {
        Self::can_cast_from(syntax.kind()).then(|| Self { syntax })
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

impl IdentifierOwner for Member {}
impl TypeIdentifierOwner for Member {}

pub trait MemberOwner: AstNode {
    fn member(&self) -> AstChildren<Member> {
        super::children(self)
    }
}
