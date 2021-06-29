mod let_statement;
pub use let_statement::*;

use super::{AstNode, AstChildren};
use crate::syn::cst::{SyntaxNode, SyntaxKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Statement {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for Statement {
    fn can_cast_from(kind: SyntaxKind) -> bool {
        match kind {
            SyntaxKind::LetStatement => true,
            _ => false,
        }
    }

    fn cast_from(syntax: SyntaxNode) -> Option<Self>
        where Self: Sized {
        Self::can_cast_from(syntax.kind()).then(|| Self { syntax })
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

impl Statement {
    pub fn kind(&self) -> StatementKind {
        match self.syntax().kind() {
            SyntaxKind::LetStatement => StatementKind::Let(LetStatement::cast_from(self.syntax().clone()).unwrap()),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum StatementKind {
    Let(LetStatement),
}

pub trait StatementsOwner: AstNode {
    fn statements(&self) -> AstChildren<Statement> {
        super::children(self)
    }
}
