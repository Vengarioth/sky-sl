use crate::hir::type_check::Ty;
use super::Block;
use rowan::{TextRange, TextSize};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FunctionKind {
    pub signature: FunctionSignature,
    pub block: Block,
    pub ty: Ty,
    pub span: TextRange,
}

impl FunctionKind {
    pub fn new(signature: FunctionSignature, block: Block, ty: Ty, span: TextRange) -> Self {
        Self {
            signature,
            block,
            ty,
            span,
        }
    }

    pub fn find_ty(&self, offset: TextSize) -> Option<Ty> {
        if self.signature.span.contains(offset) {
            for argument in &self.signature.arguments {
                if argument.span.contains(offset) {
                    return Some(argument.ty);
                }
            }

            return Some(self.ty);
        }

        if self.block.span.contains(offset) {
            return self.block.find_ty(offset);
        }

        return None;
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FunctionSignature {
    pub name: String,
    pub arguments: Vec<FunctionArgument>,
    pub return_type: Ty,
    pub span: TextRange,
}

impl FunctionSignature {
    pub fn new(name: String, arguments: Vec<FunctionArgument>, return_type: Ty, span: TextRange) -> Self {
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
    pub ty: Ty,
    pub span: TextRange,
}

impl FunctionArgument {
    pub fn new(name: String, ty: Ty, span: TextRange) -> Self {
        Self {
            name,
            ty,
            span,
        }
    }
}
