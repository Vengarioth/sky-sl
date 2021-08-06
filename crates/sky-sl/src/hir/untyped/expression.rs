use rowan::TextRange;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ExpressionKind {
    LiteralExpression(LiteralExpression),
    BinaryExpression(BinaryExpression),
    GroupExpression(GroupExpression),
    CallExpression(CallExpression),
    FieldAccessExpression(FieldAccessExpression),
    IndexExpression(IndexExpression),
    PathExpression(PathExpression),
    StructExpression(StructExpression),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct LiteralExpression {
    pub span: TextRange,
}

impl LiteralExpression {
    pub fn new(span: TextRange) -> Self {
        Self { span }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct GroupExpression {
    pub inner: Box<ExpressionKind>,
    pub span: TextRange,
}

impl GroupExpression {
    pub fn new(inner: Box<ExpressionKind>, span: TextRange) -> Self {
        Self { inner, span }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct BinaryExpression {
    pub lhs: Box<ExpressionKind>,
    pub rhs: Box<ExpressionKind>,
    pub span: TextRange,
}

impl BinaryExpression {
    pub fn new(lhs: Box<ExpressionKind>, rhs: Box<ExpressionKind>, span: TextRange) -> Self {
        Self { lhs, rhs, span }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct CallExpression {
    pub arguments: Vec<ExpressionKind>,
    pub span: TextRange,
}

impl CallExpression {
    pub fn new(arguments: Vec<ExpressionKind>, span: TextRange) -> Self {
        Self { arguments, span }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FieldAccessExpression {
    pub span: TextRange,
}

impl FieldAccessExpression {
    pub fn new(span: TextRange) -> Self {
        Self { span }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct IndexExpression {
    pub span: TextRange,
}

impl IndexExpression {
    pub fn new(span: TextRange) -> Self {
        Self { span }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PathExpression {
    pub path: String,
    pub span: TextRange,
}

impl PathExpression {
    pub fn new(path: String, span: TextRange) -> Self {
        Self { path, span }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct StructExpression {
    pub span: TextRange,
}

impl StructExpression {
    pub fn new(span: TextRange) -> Self {
        Self { span }
    }
}
