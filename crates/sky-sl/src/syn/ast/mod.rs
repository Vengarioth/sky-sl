use crate::syn::cst::*;
use std::marker::PhantomData;

mod expression;
mod statement;

mod arguments;
mod block;
mod function;
mod identifier;
mod member;
mod module_declaration;
mod module;
mod return_type;
mod root;
mod structure;
mod type_identifier;
mod use_declaration;

pub use expression::*;
pub use statement::*;

pub use arguments::*;
pub use block::*;
pub use function::*;
pub use identifier::*;
pub use member::*;
pub use module_declaration::*;
pub use module::*;
pub use return_type::*;
pub use root::*;
pub use structure::*;
pub use type_identifier::*;
pub use use_declaration::*;

pub trait AstNode: Clone {
    fn can_cast_from(kind: SyntaxKind) -> bool;
    fn cast_from(syntax: SyntaxNode) -> Option<Self> where Self: Sized;
    fn syntax(&self) -> &SyntaxNode;
}

pub trait AstToken {
    fn cast_from(token: SyntaxToken) -> Option<Self> where Self: Sized;
    fn syntax(&self) -> &SyntaxToken;
    fn text(&self) -> &str {
        self.syntax().text()
    }
}

#[derive(Debug)]
pub struct AstChildren<N> {
    inner: SyntaxNodeChildren,
    ph: PhantomData<N>,
}

impl<N> AstChildren<N> {
    fn new(parent: &SyntaxNode) -> Self {
        Self {
            inner: parent.children(),
            ph: PhantomData,
        }
    }
}

impl<N: AstNode> Iterator for AstChildren<N> {
    type Item = N;

    fn next(&mut self) -> Option<N> {
        self.inner.by_ref().find_map(N::cast_from)
    }
}

fn child<P: AstNode + ?Sized, C: AstNode>(parent: &P) -> Option<C> {
    children(parent).next()
}

fn children<P: AstNode + ?Sized, C: AstNode>(parent: &P) -> AstChildren<C> {
    AstChildren::new(parent.syntax())
}
