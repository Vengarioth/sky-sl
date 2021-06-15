use super::{AstNode, ModuleItemOwner, FunctionDefinitionOwner, StructDefinitionOwner};
use crate::syn::cst::{SyntaxNode, SyntaxKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Root {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for Root {
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

impl ModuleItemOwner for Root {}
impl FunctionDefinitionOwner for Root {}
impl StructDefinitionOwner for Root {}
