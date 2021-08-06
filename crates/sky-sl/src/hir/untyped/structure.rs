use rowan::TextRange;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct StructKind {
    pub name: String,
    pub members: Vec<StructMember>,
    pub span: TextRange,
}

impl StructKind {
    pub fn new(name: String, members: Vec<StructMember>, span: TextRange) -> Self {
        Self {
            name,
            members,
            span,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct StructMember {
    pub name: String,
    pub ty_name: String,
    pub span: TextRange,
}

impl StructMember {
    pub fn new(name: String, ty_name: String, span: TextRange) -> Self {
        Self {
            name,
            ty_name,
            span,
        }
    }
}
