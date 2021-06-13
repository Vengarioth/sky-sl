use crate::syn::cst::*;

pub trait AstNode {
    fn can_cast_from(kind: SyntaxKind) -> bool;
    fn cast_from(syntax: SyntaxNode) -> Option<Self> where Self: Sized;
    fn syntax(&self) -> &SyntaxNode;
}

#[derive(Debug)]
pub struct FunctionDeclaration {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for FunctionDeclaration {
    fn can_cast_from(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::Fn
    }

    fn cast_from(syntax: SyntaxNode) -> Option<Self>
        where Self: Sized
    {
        Self::can_cast_from(syntax.kind()).then(|| FunctionDeclaration {
            syntax
        })
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
