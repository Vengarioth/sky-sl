use thiserror::*;

#[derive(Debug, Error, Eq, PartialEq, Clone)]
pub enum LowerToHirError {

    #[error("Missing function signature")]
    MissingFunctionSignature,

    #[error("Incomplete function signature")]
    IncompleteFunctionSignature,
}
