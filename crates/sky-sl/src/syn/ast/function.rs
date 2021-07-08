use super::{AstChildren, AstNode, IdentifierOwner, ArgumentListOwner, ReturnTypeOwner, BlockDefinitionOwner};
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

impl FunctionSignatureOwner for FunctionDefinition {}
impl BlockDefinitionOwner for FunctionDefinition {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FunctionSignature {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for FunctionSignature {
    fn can_cast_from(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::FnSignature
    }

    fn cast_from(syntax: SyntaxNode) -> Option<Self>
        where Self: Sized {
        Self::can_cast_from(syntax.kind()).then(|| Self { syntax })
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

pub trait FunctionSignatureOwner: AstNode {
    fn signature(&self) -> Option<FunctionSignature> {
        super::child(self)
    }
}

impl IdentifierOwner for FunctionSignature {}
impl ArgumentListOwner for FunctionSignature {}
impl ReturnTypeOwner for FunctionSignature {}
