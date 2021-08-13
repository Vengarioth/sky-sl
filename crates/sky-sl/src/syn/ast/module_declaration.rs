use super::{AstNode, AstChildren, IdentifierOwner};
use crate::syn::cst::{SyntaxNode, SyntaxKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ModuleDeclaration {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for ModuleDeclaration {
    fn can_cast_from(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::Module
    }

    fn cast_from(syntax: SyntaxNode) -> Option<Self>
        where Self: Sized {
        Self::can_cast_from(syntax.kind()).then(|| Self { syntax })
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

pub trait ModuleDeclarationOwner: AstNode {
    fn modules(&self) -> AstChildren<ModuleDeclaration> {
        super::children(self)
    }
}

impl IdentifierOwner for ModuleDeclaration {}
