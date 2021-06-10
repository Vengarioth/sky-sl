use crate::syn::NodeOrToken;
use super::SyntaxKind;
use text_size::TextSize;
use std::sync::Arc;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct GreenNode {
    pub kind: SyntaxKind,
    // pub length: TextSize,
    pub children: Vec<Arc<GreenChild>>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct GreenToken {
    pub kind: SyntaxKind,
    pub length: TextSize,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum GreenChild {
    Node(GreenNode),
    Token(GreenToken),
}

pub struct GreenNodeHead {

}
