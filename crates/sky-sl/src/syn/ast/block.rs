use super::{AstNode, StatementsOwner};
use crate::syn::cst::{SyntaxNode, SyntaxKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BlockDefinition {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for BlockDefinition {
    fn can_cast_from(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::Block
    }

    fn cast_from(syntax: SyntaxNode) -> Option<Self>
        where Self: Sized {
        Self::can_cast_from(syntax.kind()).then(|| Self { syntax })
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

pub trait BlockDefinitionOwner: AstNode {
    fn block_definition(&self) -> Option<BlockDefinition> {
        super::child(self)
    }
}

impl StatementsOwner for BlockDefinition {}
