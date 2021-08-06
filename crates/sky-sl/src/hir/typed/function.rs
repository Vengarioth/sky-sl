use crate::hir::type_check::Ty;
use super::Block;
use rowan::TextRange;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FunctionKind {
    pub ty: Ty,
    pub signature: FunctionSignature,
    pub block: Block,
    pub span: TextRange,
}

impl FunctionKind {
    pub fn new(ty: Ty, signature: FunctionSignature, block: Block, span: TextRange) -> Self {
        Self {
            ty,
            signature,
            block,
            span,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FunctionSignature {
    pub name: String,
    pub arguments: Vec<FunctionArgument>,
    pub return_type: Option<String>,
    pub span: TextRange,
}

impl FunctionSignature {
    pub fn new(name: String, arguments: Vec<FunctionArgument>, return_type: Option<String>, span: TextRange) -> Self {
        Self {
            name,
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
    pub ty: Ty,
    pub span: TextRange,
}

impl FunctionArgument {
    pub fn new(name: String, ty_name: String, ty: Ty, span: TextRange) -> Self {
        Self {
            name,
            ty_name,
            ty,
            span,
        }
    }
}
