use super::super::{AstNode, Expression, ExpressionsOwner};
use crate::syn::cst::{SyntaxNode, SyntaxKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BinaryExpression {
    pub(crate) syntax: SyntaxNode,
}

impl BinaryExpression {
    pub fn lhs(&self) -> Option<Expression> {
        self.expressions().nth(0)
    }

    pub fn rhs(&self) -> Option<Expression> {
        self.expressions().nth(1)
    }
}

impl AstNode for BinaryExpression {
    fn can_cast_from(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::BinaryExpression
    }

    fn cast_from(syntax: SyntaxNode) -> Option<Self>
        where Self: Sized {
        Self::can_cast_from(syntax.kind()).then(|| Self { syntax })
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

impl ExpressionsOwner for BinaryExpression {}
impl OperatorOwner for BinaryExpression {}

// TODO use cst::Operator ?
pub enum OperatorKind {
    /// Assignment operator
    Assign,

    /// Arithmetic Add operator
    Add,

    /// Arithmetic Subtract operator
    Subtract,
    
    /// Arithmetic Multiply operator
    Multiply,
    
    /// Arithmetic Divide operator
    Divide,
    
    /// Remainder (modulo) operator
    Remainder,

    /// Logical AND operator
    And,

    /// Logical OR operator
    Or,

    /// Logical XOR operator
    XOr,

    // TODO left shift
    // TODO right shift
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Operator {
    pub(crate) syntax: SyntaxNode,
}

impl Operator {
    pub fn kind(&self) -> OperatorKind {
        todo!()
    }
}

impl AstNode for Operator {
    fn can_cast_from(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::Operator
    }

    fn cast_from(syntax: SyntaxNode) -> Option<Self>
        where Self: Sized {
        Self::can_cast_from(syntax.kind()).then(|| Self { syntax })
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

pub trait OperatorOwner: AstNode {
    fn operator(&self) -> Option<Operator> {
        super::super::child(self)
    }
}
