use tower_lsp::lsp_types::*;
use std::ops::Deref;

pub struct ModifierIndex {
    flag: u32,
    token_modifier: SemanticTokenModifier,
}

impl ModifierIndex {
    // TODO we can refactor this with a macro when https://github.com/rust-lang/rust/issues/83527 lands on stable
    pub const DECLARATION: ModifierIndex = ModifierIndex::new(1, "declaration");
    pub const DEFINITION: ModifierIndex = ModifierIndex::new(2, "definition");
    pub const READONLY: ModifierIndex = ModifierIndex::new(4, "readonly");
    pub const STATIC: ModifierIndex = ModifierIndex::new(8, "static");
    pub const DEPRECATED: ModifierIndex = ModifierIndex::new(16, "deprecated");
    pub const ABSTRACT: ModifierIndex = ModifierIndex::new(32, "abstract");
    pub const ASYNC: ModifierIndex = ModifierIndex::new(64, "async");
    pub const MODIFICATION: ModifierIndex = ModifierIndex::new(128, "modification");
    pub const DOCUMENTATION: ModifierIndex = ModifierIndex::new(256, "documentation");
    pub const DEFAULT_LIBRARY: ModifierIndex = ModifierIndex::new(512, "defaultLibrary");
    // Don't forget to add it to Self::legend as well

    pub const NONE: ModifierIndex = ModifierIndex::new(0, "none");
    
    const fn new(flag: u32, tag: &'static str) -> Self {
        Self {
            flag,
            token_modifier: SemanticTokenModifier::new(tag),
        }
    }

    pub fn token_modifier(&self) -> SemanticTokenModifier {
        self.token_modifier.clone()
    }

    pub fn legend() -> Vec<SemanticTokenModifier> {
        vec![
            Self::DECLARATION.token_modifier(),
            Self::DEFINITION.token_modifier(),
            Self::READONLY.token_modifier(),
            Self::STATIC.token_modifier(),
            Self::DEPRECATED.token_modifier(),
            Self::ABSTRACT.token_modifier(),
            Self::ASYNC.token_modifier(),
            Self::MODIFICATION.token_modifier(),
            Self::DOCUMENTATION.token_modifier(),
            Self::DEFAULT_LIBRARY.token_modifier(),
        ]
    }
}

impl Deref for ModifierIndex {
    type Target = u32;

    fn deref(&self) -> &u32 {
        &self.flag
    }
}
