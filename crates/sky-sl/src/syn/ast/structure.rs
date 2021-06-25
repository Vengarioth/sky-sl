use super::{AstChildren, AstNode, IdentifierOwner, MemberListOwner};
use crate::syn::cst::{SyntaxNode, SyntaxKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StructDefinition {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for StructDefinition {
    fn can_cast_from(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::Struct
    }

    fn cast_from(syntax: SyntaxNode) -> Option<Self>
        where Self: Sized {
        Self::can_cast_from(syntax.kind()).then(|| Self { syntax })
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

pub trait StructDefinitionOwner: AstNode {
    fn struct_definitions(&self) -> AstChildren<StructDefinition> {
        super::children(self)
    }
}

impl IdentifierOwner for StructDefinition {}
impl MemberListOwner for StructDefinition {}
