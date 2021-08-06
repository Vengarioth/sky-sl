use super::StatementKind;
use rowan::TextRange;

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
}
