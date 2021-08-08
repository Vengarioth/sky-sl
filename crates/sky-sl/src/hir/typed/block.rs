use crate::hir::type_check::Ty;
use super::StatementKind;
use rowan::{TextRange, TextSize};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Block {
    pub statements: Vec<StatementKind>,
    pub span: TextRange,
}

impl Block {
    pub fn new(statements: Vec<StatementKind>, span: TextRange) -> Self {
        Self {
            statements,
            span,
        }
    }

    pub fn find_ty(&self, offset: TextSize) -> Option<Ty> {
        for statement in &self.statements {
            if statement.span().contains(offset) {
                return statement.find_ty(offset);
            }
        }

        return None;
    }
}
