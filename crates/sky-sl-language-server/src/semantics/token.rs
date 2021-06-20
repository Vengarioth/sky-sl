use tower_lsp::lsp_types::*;
    
pub const NAMESPACE: SemanticTokenType = SemanticTokenType::new("namespace");
pub const KEYWORD: SemanticTokenType = SemanticTokenType::new("keyword");
pub const FUNCTION: SemanticTokenType = SemanticTokenType::new("function");
pub const STRUCT: SemanticTokenType = SemanticTokenType::new("struct");
pub const PARAMETER: SemanticTokenType = SemanticTokenType::new("parameter");

// scopes: (), {}, []
pub const PARENTHESES: SemanticTokenType = SemanticTokenType::new("parentheses");
pub const BRACE: SemanticTokenType = SemanticTokenType::new("brace");
pub const BRACKET: SemanticTokenType = SemanticTokenType::new("bracket");

// punctuation
pub const COLON: SemanticTokenType = SemanticTokenType::new("colon");

pub const OPERATOR: SemanticTokenType = SemanticTokenType::new("operator");

// unused
// pub const TYPE: SemanticTokenType = SemanticTokenType::new("type");
// pub const CLASS: SemanticTokenType = SemanticTokenType::new("class");
// pub const ENUM: SemanticTokenType = SemanticTokenType::new("enum");
// pub const INTERFACE: SemanticTokenType = SemanticTokenType::new("interface");
// pub const TYPE_PARAMETER: SemanticTokenType = SemanticTokenType::new("typeParameter");
// pub const VARIABLE: SemanticTokenType = SemanticTokenType::new("variable");
// pub const PROPERTY: SemanticTokenType = SemanticTokenType::new("property");
// pub const ENUM_MEMBER: SemanticTokenType = SemanticTokenType::new("enumMember");
// pub const EVENT: SemanticTokenType = SemanticTokenType::new("event");
// pub const METHOD: SemanticTokenType = SemanticTokenType::new("method");
// pub const MACRO: SemanticTokenType = SemanticTokenType::new("macro");
// pub const MODIFIER: SemanticTokenType = SemanticTokenType::new("modifier");
// pub const COMMENT: SemanticTokenType = SemanticTokenType::new("comment");
// pub const STRING: SemanticTokenType = SemanticTokenType::new("string");
// pub const NUMBER: SemanticTokenType = SemanticTokenType::new("number");
// pub const REGEXP: SemanticTokenType = SemanticTokenType::new("regexp");

pub fn get_semantic_token_types() -> Vec<SemanticTokenType> {
    vec![
        NAMESPACE,
        KEYWORD,
        FUNCTION,
        STRUCT,
        PARAMETER,
        PARENTHESES,
        BRACE,
        BRACKET,
        COLON,
        OPERATOR,
    ]
}
