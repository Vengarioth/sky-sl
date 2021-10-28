use super::{AstChildren, AstNode, ExpressionOwner, NameOwner, PathOwner};
use crate::syn::cst::{SyntaxNode, SyntaxKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LayoutDefinition {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for LayoutDefinition {
    fn can_cast_from(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::Layout
    }

    fn cast_from(syntax: SyntaxNode) -> Option<Self>
        where Self: Sized {
        Self::can_cast_from(syntax.kind()).then(|| Self { syntax })
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

pub trait LayoutDefinitionOwner: AstNode {
    fn struct_definitions(&self) -> AstChildren<LayoutDefinition> {
        super::children(self)
    }
}

impl NameOwner for LayoutDefinition {}
impl LayoutMemberListOwner for LayoutDefinition {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LayoutMemberList {
    syntax: SyntaxNode,
}

impl AstNode for LayoutMemberList {
    fn can_cast_from(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::LayoutMemberList
    }

    fn cast_from(syntax: SyntaxNode) -> Option<Self> where Self: Sized {
        Self::can_cast_from(syntax.kind()).then(|| Self { syntax })
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

pub trait LayoutMemberListOwner: AstNode {
    fn layout_member_list(&self) -> Option<LayoutMemberList> {
        super::child(self)
    }
}

impl LayoutMemberOwner for LayoutMemberList {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LayoutMember {
    syntax: SyntaxNode,
}

impl AstNode for LayoutMember {
    fn can_cast_from(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::LayoutMember
    }

    fn cast_from(syntax: SyntaxNode) -> Option<Self> where Self: Sized {
        Self::can_cast_from(syntax.kind()).then(|| Self { syntax })
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

impl NameOwner for LayoutMember {}
impl PathOwner for LayoutMember {}
impl BindingIndexOwner for LayoutMember {}
impl BindingKindOwner for LayoutMember {}

pub trait LayoutMemberOwner: AstNode {
    fn layout_member(&self) -> AstChildren<LayoutMember> {
        super::children(self)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BindingIndex {
    syntax: SyntaxNode,
}

impl AstNode for BindingIndex {
    fn can_cast_from(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::BindingIndex
    }

    fn cast_from(syntax: SyntaxNode) -> Option<Self> where Self: Sized {
        Self::can_cast_from(syntax.kind()).then(|| Self { syntax })
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

pub trait BindingIndexOwner: AstNode {
    fn binding_index(&self) -> Option<BindingIndex> {
        super::child(self)
    }
}

impl ExpressionOwner for BindingIndex {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BindingKind {
    syntax: SyntaxNode,
}

impl AstNode for BindingKind {
    fn can_cast_from(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::BindingKind
    }

    fn cast_from(syntax: SyntaxNode) -> Option<Self> where Self: Sized {
        Self::can_cast_from(syntax.kind()).then(|| Self { syntax })
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

pub trait BindingKindOwner: AstNode {
    fn binding_kind(&self) -> Option<BindingKind> {
        super::child(self)
    }
}
