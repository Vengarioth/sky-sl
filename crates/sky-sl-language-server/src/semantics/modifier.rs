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

pub fn get_semantic_modifier_types() -> Vec<SemanticTokenModifier> {
    vec![
        DECLARATION,
        DEFINITION,
        READONLY,
        STATIC,
        DEPRECATED,
        ABSTRACT,
        ASYNC,
        MODIFICATION,
        DOCUMENTATION,
        DEFAULT_LIBRARY,
    ]
}
