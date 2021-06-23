use tower_lsp::lsp_types::*;
use sky_sl::syn::{ast::*, cst::*};

pub mod token;
pub mod modifier;

mod builder;
use builder::*;

// TODO refactor DeltaEncoder into SemanticTokensBuilder that also havs the LineIndex internally

pub fn get_legend() -> SemanticTokensLegend {
    SemanticTokensLegend {
        token_types: token::get_semantic_token_types(),
        token_modifiers: modifier::get_semantic_modifier_types(),
    }
}

pub fn get_semantic_tokens(root: Root, line_index: &LineIndex) -> SemanticTokens {
    let mut builder = SemanticTokensBuilder::new(line_index);

    dbg!(&root);

    visit_root(root, &mut builder);

    SemanticTokens {
        result_id: None,
        data: builder.finish(),
    }
}

fn visit_root(root: Root, builder: &mut SemanticTokensBuilder) {
    for module_item in root.module_items() {
        match module_item.kind() {
            ModuleItemKind::FunctionDefinition(function_definition) => {
                visit_function_definition(function_definition, builder);
            },
            ModuleItemKind::StructDefinition(struct_definition) => {
                visit_struct_definition(struct_definition, builder);
            },
        }
    }
}

fn visit_function_definition(function_definition: FunctionDefinition, builder: &mut SemanticTokensBuilder) {
    if let Some(keyword) = function_definition.syntax().first_token() {
        builder.build_token(keyword.text_range(), 1, 0);
    }

    if let Some(identifier) = function_definition.identifier() {
        let syntax = identifier.syntax();
        builder.build_token(syntax.text_range(), 2, 1);
    }

    if let Some(argument_list) = function_definition.argument_list() {
        visit_argument_list(argument_list, builder)
    }
}

fn visit_argument_list(argument_list: ArgumentList, builder: &mut SemanticTokensBuilder) {
    for argument in argument_list.arguments() {
        if let Some(identifier) = argument.identifier() {
            builder.build_token(identifier.syntax().text_range(), 4, 0);
        }
    }
}

fn visit_struct_definition(struct_definition: StructDefinition, builder: &mut SemanticTokensBuilder) {
    if let Some(keyword) = struct_definition.syntax().first_token() {
        builder.build_token(keyword.text_range(), 1, 0);
    }

    if let Some(identifier) = struct_definition.identifier() {
        let syntax = identifier.syntax();
        builder.build_token(syntax.text_range(), 3, 0);
    }
}
