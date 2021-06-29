use super::{AstChildren, AstNode, IdentifierOwner, ArgumentListOwner, BlockDefinitionOwner};
use crate::syn::cst::{SyntaxNode, SyntaxKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FunctionDefinition {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for FunctionDefinition {
    fn can_cast_from(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::Fn
    }

    fn cast_from(syntax: SyntaxNode) -> Option<Self>
        where Self: Sized {
        Self::can_cast_from(syntax.kind()).then(|| Self { syntax })
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

pub trait FunctionDefinitionOwner: AstNode {
    fn function_definitions(&self) -> AstChildren<FunctionDefinition> {
        super::children(self)
    }
}

impl IdentifierOwner for FunctionDefinition {}
impl ArgumentListOwner for FunctionDefinition {}
impl BlockDefinitionOwner for FunctionDefinition {}
