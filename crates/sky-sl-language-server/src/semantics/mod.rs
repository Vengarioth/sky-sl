use tower_lsp::lsp_types::*;
use sky_sl::syn::{ast::*, cst::*};

pub mod token;
pub mod modifier;

mod delta_encoder;
pub use delta_encoder::*;

pub fn get_legend() -> SemanticTokensLegend {
    SemanticTokensLegend {
        token_types: token::get_semantic_token_types(),
        token_modifiers: modifier::get_semantic_modifier_types(),
    }
}

pub fn get_semantic_tokens(root: Root, line_index: &LineIndex) -> SemanticTokens {

    let mut tokens = Vec::new();
    let mut delta_encoder = DeltaEncoder::new();
    visit_root(root, line_index, &mut delta_encoder, &mut tokens);

    SemanticTokens {
        result_id: None,
        data: tokens,
    }
}

fn visit_root(root: Root, line_index: &LineIndex, delta_encoder: &mut DeltaEncoder, tokens: &mut Vec<SemanticToken>) {
    for module_item in root.module_items() {
        match module_item.kind() {
            ModuleItemKind::FunctionDefinition(function_definition) => {
                visit_function_definition(function_definition, line_index, delta_encoder, tokens);
            },
            ModuleItemKind::StructDefinition(struct_definition) => {
                visit_struct_definition(struct_definition, line_index, delta_encoder, tokens);
            },
        }
    }
}

fn visit_function_definition(function_definition: FunctionDefinition, line_index: &LineIndex, delta_encoder: &mut DeltaEncoder, tokens: &mut Vec<SemanticToken>) {
    if let Some(keyword) = function_definition.syntax().first_token() {
        let range = line_index.find_range(keyword.text_range());
        tokens.push(delta_encoder.create_next(range, 1, 0));
    }

    if let Some(identifier) = function_definition.identifier() {
        let syntax = identifier.syntax();
        let range = line_index.find_range(syntax.text_range());
        tokens.push(delta_encoder.create_next(range, 2, 0));
    }
}

fn visit_struct_definition(struct_definition: StructDefinition, line_index: &LineIndex, delta_encoder: &mut DeltaEncoder, tokens: &mut Vec<SemanticToken>) {
    if let Some(keyword) = struct_definition.syntax().first_token() {
        let range = line_index.find_range(keyword.text_range());
        tokens.push(delta_encoder.create_next(range, 1, 0));
    }

    if let Some(identifier) = struct_definition.identifier() {
        let syntax = identifier.syntax();
        let range = line_index.find_range(syntax.text_range());
        tokens.push(delta_encoder.create_next(range, 3, 0));
    }
}
