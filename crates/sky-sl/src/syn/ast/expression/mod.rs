use super::{AstNode, AstChildren};
use crate::syn::cst::{SyntaxNode, SyntaxKind};

mod binary_expression;
mod group_expression;
mod literal_expression;
mod path_expression;

pub use binary_expression::*;
pub use group_expression::*;
pub use literal_expression::*;
pub use path_expression::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Expression {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for Expression {
    fn can_cast_from(kind: SyntaxKind) -> bool {
        match kind {
            SyntaxKind::LiteralExpression | SyntaxKind::GroupExpression | SyntaxKind::BinaryExpression | SyntaxKind::PathExpression => true,
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

impl Expression {
    pub fn kind(&self) -> ExpressionKind {
        match self.syntax().kind() {
            SyntaxKind::LiteralExpression => ExpressionKind::LiteralExpression(LiteralExpression::cast_from(self.syntax().clone()).unwrap()),
            SyntaxKind::GroupExpression => ExpressionKind::GroupExpression(GroupExpression::cast_from(self.syntax().clone()).unwrap()),
            SyntaxKind::BinaryExpression => ExpressionKind::BinaryExpression(BinaryExpression::cast_from(self.syntax().clone()).unwrap()),
            SyntaxKind::PathExpression => ExpressionKind::PathExpression(PathExpression::cast_from(self.syntax().clone()).unwrap()),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ExpressionKind {
    LiteralExpression(LiteralExpression),
    GroupExpression(GroupExpression),
    BinaryExpression(BinaryExpression),
    PathExpression(PathExpression),
}

pub trait ExpressionOwner: AstNode {
    fn expression(&self) -> Option<Expression> {
        super::child(self)
    }
}

pub trait ExpressionsOwner: AstNode {
    fn expressions(&self) -> AstChildren<Expression> {
        super::children(self)
    }
}
