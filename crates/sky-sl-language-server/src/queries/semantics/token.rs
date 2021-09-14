use tower_lsp::lsp_types::*;
use std::ops::Deref;

pub struct TokenIndex {
    index: u32,
    token_type: SemanticTokenType,
}

impl TokenIndex {
    // TODO we can refactor this with a macro when https://github.com/rust-lang/rust/issues/83527 lands on stable
    pub const NAMESPACE: TokenIndex = TokenIndex::new(0, "namespace");
    pub const KEYWORD: TokenIndex = TokenIndex::new(1, "keyword");
    pub const FUNCTION: TokenIndex = TokenIndex::new(2, "function");
    pub const STRUCT: TokenIndex = TokenIndex::new(3, "struct");
    pub const PARAMETER: TokenIndex = TokenIndex::new(4, "parameter");
    pub const PROPERTY: TokenIndex = TokenIndex::new(5, "property");
    pub const TYPE: TokenIndex = TokenIndex::new(6, "type");
    pub const VARIABLE: TokenIndex = TokenIndex::new(7, "variable");
    pub const NUMBER: TokenIndex = TokenIndex::new(8, "number");
    // Don't forget to add it to Self::legend as well

    const fn new(index: u32, tag: &'static str) -> Self {
        Self {
            index,
            token_type: SemanticTokenType::new(tag),
        }
    }

    pub fn token_type(&self) -> SemanticTokenType {
        self.token_type.clone()
    }

    pub fn legend() -> Vec<SemanticTokenType> {
        vec![
            Self::NAMESPACE.token_type(),
            Self::KEYWORD.token_type(),
            Self::FUNCTION.token_type(),
            Self::STRUCT.token_type(),
            Self::PARAMETER.token_type(),
            Self::PROPERTY.token_type(),
            Self::TYPE.token_type(),
            Self::VARIABLE.token_type(),
            Self::NUMBER.token_type(),
        ]
    }
}

impl Deref for TokenIndex {
    type Target = u32;

    fn deref(&self) -> &u32 {
        &self.index
    }
}
