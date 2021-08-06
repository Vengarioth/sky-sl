use thiserror::*;

#[derive(Debug, Error, Eq, PartialEq, Clone)]
pub enum LowerToHirError {

    #[error("Missing function signature")]
    MissingFunctionSignature,

    #[error("Incomplete function signature")]
    IncompleteFunctionSignature,

    #[error("Incomplete function body")]
    IncompleteFunctionBody,

    #[error("Incomplete struct declaration")]
    IncompleteStructDeclaration,

    #[error("Incomplete statement")]
    IncompleteStatement,

    #[error("Incomplete expression")]
    IncompleteExpression,
}
