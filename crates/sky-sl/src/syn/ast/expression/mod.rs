use super::{AstChildren, AstNode};
use crate::syn::cst::{SyntaxKind, SyntaxNode};

mod binary_expression;
mod call_expression;
mod field_access_expression;
mod group_expression;
mod index_expression;
mod literal_expression;
mod path_expression;

pub use binary_expression::*;
pub use call_expression::*;
pub use field_access_expression::*;
pub use group_expression::*;
pub use index_expression::*;
pub use literal_expression::*;
pub use path_expression::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Expression {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for Expression {
    fn can_cast_from(kind: SyntaxKind) -> bool {
        match kind {
            SyntaxKind::LiteralExpression
            | SyntaxKind::GroupExpression
            | SyntaxKind::BinaryExpression
            | SyntaxKind::CallExpression
            | SyntaxKind::FieldAccessExpression
            | SyntaxKind::IndexExpression
            | SyntaxKind::PathExpression => true,
            _ => false,
        }
    }

    fn cast_from(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        Self::can_cast_from(syntax.kind()).then(|| Self { syntax })
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

impl Expression {
    pub fn kind(&self) -> ExpressionKind {
        match self.syntax().kind() {
            SyntaxKind::LiteralExpression => ExpressionKind::LiteralExpression(
                LiteralExpression::cast_from(self.syntax().clone()).unwrap(),
            ),
            SyntaxKind::GroupExpression => ExpressionKind::GroupExpression(
                GroupExpression::cast_from(self.syntax().clone()).unwrap(),
            ),
            SyntaxKind::BinaryExpression => ExpressionKind::BinaryExpression(
                BinaryExpression::cast_from(self.syntax().clone()).unwrap(),
            ),
            SyntaxKind::CallExpression => ExpressionKind::CallExpression(
                CallExpression::cast_from(self.syntax().clone()).unwrap(),
            ),
            SyntaxKind::FieldAccessExpression => ExpressionKind::FieldAccessExpression(
                FieldAccessExpression::cast_from(self.syntax().clone()).unwrap(),
            ),
            SyntaxKind::IndexExpression => ExpressionKind::IndexExpression(
                IndexExpression::cast_from(self.syntax().clone()).unwrap(),
            ),
            SyntaxKind::PathExpression => ExpressionKind::PathExpression(
                PathExpression::cast_from(self.syntax().clone()).unwrap(),
            ),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ExpressionKind {
    LiteralExpression(LiteralExpression),
    GroupExpression(GroupExpression),
    BinaryExpression(BinaryExpression),
    CallExpression(CallExpression),
    FieldAccessExpression(FieldAccessExpression),
    IndexExpression(IndexExpression),
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
