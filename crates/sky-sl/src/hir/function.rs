use rowan::TextRange;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FunctionKind {
    pub signature: FunctionSignature,
    pub span: TextRange,
}

impl FunctionKind {
    pub fn new(signature: FunctionSignature, span: TextRange) -> Self {
        Self {
            signature,
            span,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FunctionSignature {
    pub arguments: Vec<FunctionArgument>,
    pub return_type: Option<String>,
    pub span: TextRange,
}

impl FunctionSignature {
    pub fn new(arguments: Vec<FunctionArgument>, return_type: Option<String>, span: TextRange) -> Self {
        Self {
            arguments,
            return_type,
            span,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FunctionArgument {
    pub name: String,
    pub ty_name: String,
    pub span: TextRange,
}

impl FunctionArgument {
    pub fn new(name: String, ty_name: String, span: TextRange) -> Self {
        Self {
            name,
            ty_name,
            span,
        }
    }
}
