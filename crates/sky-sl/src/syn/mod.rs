use std::marker::PhantomData;
use rowan::GreenNode;

pub mod ast;
pub mod cst;

#[derive(Debug, Eq, PartialEq)]
pub struct Parse<T> {
    green: GreenNode,
    phantom: PhantomData<T>,
}

impl<T: ast::AstNode> Parse<T> {
    pub fn new(green: GreenNode) -> Self {
        Self {
            green,
            phantom: PhantomData,
        }
    }

    pub fn syntax_node(&self) -> cst::SyntaxNode {
        cst::SyntaxNode::new_root(self.green.clone())
    }

    pub fn tree(&self) -> T {
        T::cast_from(self.syntax_node()).unwrap()
    }
}

impl<T> Clone for Parse<T> {
    fn clone(&self) -> Self {
        Self {
            green: self.green.clone(),
            phantom: PhantomData,
        }
    }
}

// Safety: GreenNode is Send + Sync, only the T in PhantomData<T> can be !send and/or !sync, which is fine since it is never stored in Parse<T>
unsafe impl<T> Send for Parse<T> {}
unsafe impl<T> Sync for Parse<T> {}
