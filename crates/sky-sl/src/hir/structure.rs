use rowan::TextRange;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct StructureKind {
    pub span: TextRange,
}

impl StructureKind {
    pub fn new(span: TextRange) -> Self {
        Self {
            span,
        }
    }
}
