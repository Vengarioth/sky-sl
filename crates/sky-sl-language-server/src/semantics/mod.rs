#[allow(dead_code)]
pub mod token {
    use tower_lsp::lsp_types::*;

    pub const NAMESPACE: SemanticTokenType = SemanticTokenType::new("namespace");
    pub const TYPE: SemanticTokenType = SemanticTokenType::new("type");
    pub const CLASS: SemanticTokenType = SemanticTokenType::new("class");
    pub const ENUM: SemanticTokenType = SemanticTokenType::new("enum");
    pub const INTERFACE: SemanticTokenType = SemanticTokenType::new("interface");
    pub const STRUCT: SemanticTokenType = SemanticTokenType::new("struct");
    pub const TYPE_PARAMETER: SemanticTokenType = SemanticTokenType::new("typeParameter");
    pub const PARAMETER: SemanticTokenType = SemanticTokenType::new("parameter");
    pub const VARIABLE: SemanticTokenType = SemanticTokenType::new("variable");
    pub const PROPERTY: SemanticTokenType = SemanticTokenType::new("property");
    pub const ENUM_MEMBER: SemanticTokenType = SemanticTokenType::new("enumMember");
    pub const EVENT: SemanticTokenType = SemanticTokenType::new("event");
    pub const FUNCTION: SemanticTokenType = SemanticTokenType::new("function");
    pub const METHOD: SemanticTokenType = SemanticTokenType::new("method");
    pub const MACRO: SemanticTokenType = SemanticTokenType::new("macro");
    pub const KEYWORD: SemanticTokenType = SemanticTokenType::new("keyword");
    pub const MODIFIER: SemanticTokenType = SemanticTokenType::new("modifier");
    pub const COMMENT: SemanticTokenType = SemanticTokenType::new("comment");
    pub const STRING: SemanticTokenType = SemanticTokenType::new("string");
    pub const NUMBER: SemanticTokenType = SemanticTokenType::new("number");
    pub const REGEXP: SemanticTokenType = SemanticTokenType::new("regexp");
    pub const OPERATOR: SemanticTokenType = SemanticTokenType::new("operator");
}

#[allow(dead_code)]
pub mod token_modifier {
    use tower_lsp::lsp_types::*;

    pub const DECLARATION: SemanticTokenModifier = SemanticTokenModifier::new("declaration");
    pub const DEFINITION: SemanticTokenModifier = SemanticTokenModifier::new("definition");
    pub const READONLY: SemanticTokenModifier = SemanticTokenModifier::new("readonly");
    pub const STATIC: SemanticTokenModifier = SemanticTokenModifier::new("static");
    pub const DEPRECATED: SemanticTokenModifier = SemanticTokenModifier::new("deprecated");
    pub const ABSTRACT: SemanticTokenModifier = SemanticTokenModifier::new("abstract");
    pub const ASYNC: SemanticTokenModifier = SemanticTokenModifier::new("async");
    pub const MODIFICATION: SemanticTokenModifier = SemanticTokenModifier::new("modification");
    pub const DOCUMENTATION: SemanticTokenModifier = SemanticTokenModifier::new("documentation");
    pub const DEFAULT_LIBRARY: SemanticTokenModifier = SemanticTokenModifier::new("defaultLibrary");
}
