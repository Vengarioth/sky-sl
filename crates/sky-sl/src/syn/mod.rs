use rowan::GreenNode;
use crate::parser::SyntaxError;
use std::marker::PhantomData;
use std::sync::Arc;

pub mod db;
pub mod ast;
pub mod cst;

#[derive(Debug, Eq, PartialEq)]
pub struct Parse<T> {
    green: GreenNode,
    phantom: PhantomData<T>,
    errors: Arc<Vec<SyntaxError>>,
}

impl<T: ast::AstNode> Parse<T> {
    pub fn new(green: GreenNode, errors: Vec<SyntaxError>) -> Self {
        Self {
            green,
            phantom: PhantomData,
            errors: Arc::new(errors),
        }
    }

    pub fn syntax_node(&self) -> cst::SyntaxNode {
        cst::SyntaxNode::new_root(self.green.clone())
    }

    pub fn tree(&self) -> T {
        T::cast_from(self.syntax_node()).unwrap()
    }

    pub fn errors(&self) -> &[SyntaxError] {
        &self.errors
    }
}

impl<T> Clone for Parse<T> {
    fn clone(&self) -> Self {
        Self {
            green: self.green.clone(),
            phantom: PhantomData,
            errors: Arc::clone(&self.errors),
        }
    }
}

// Safety: GreenNode is Send + Sync, only the T in PhantomData<T> can be !send and/or !sync, which is fine since it is never stored in Parse<T>
unsafe impl<T> Send for Parse<T> {}
unsafe impl<T> Sync for Parse<T> {}
