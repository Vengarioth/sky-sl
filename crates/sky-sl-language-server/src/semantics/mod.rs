use tower_lsp::lsp_types::*;
use sky_sl::syn::{ast::*, cst::*};

mod builder;
pub mod token;
pub mod modifier;

use builder::*;
use token::TokenIndex;
use modifier::ModifierIndex;

pub fn get_legend() -> SemanticTokensLegend {
    SemanticTokensLegend {
        token_types: token::TokenIndex::legend(),
        token_modifiers: ModifierIndex::legend(),
    }
}

pub fn get_semantic_tokens(root: Root, line_index: &LineIndex) -> SemanticTokens {
    let mut builder = SemanticTokensBuilder::new(line_index);

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
        builder.build_token(keyword.text_range(), *TokenIndex::KEYWORD, *ModifierIndex::NONE);
    }

    if let Some(identifier) = function_definition.identifier() {
        let syntax = identifier.syntax();
        builder.build_token(syntax.text_range(), *TokenIndex::FUNCTION, *ModifierIndex::DECLARATION);
    }

    if let Some(argument_list) = function_definition.argument_list() {
        visit_argument_list(argument_list, builder)
    }
}

fn visit_argument_list(argument_list: ArgumentList, builder: &mut SemanticTokensBuilder) {
    for argument in argument_list.arguments() {
        if let Some(identifier) = argument.identifier() {
            builder.build_token(identifier.syntax().text_range(), *TokenIndex::PARAMETER, *ModifierIndex::DECLARATION);
        }
    }
}

fn visit_struct_definition(struct_definition: StructDefinition, builder: &mut SemanticTokensBuilder) {
    if let Some(keyword) = struct_definition.syntax().first_token() {
        builder.build_token(keyword.text_range(), *TokenIndex::KEYWORD, *ModifierIndex::NONE);
    }

    if let Some(identifier) = struct_definition.identifier() {
        let syntax = identifier.syntax();
        builder.build_token(syntax.text_range(), *TokenIndex::STRUCT, *ModifierIndex::NONE);
    }

    if let Some(member_list) = struct_definition.member_list() {
        for member in member_list.member() {
            visit_struct_member(member, builder);
        }
    }
}

fn visit_struct_member(member: Member, builder: &mut SemanticTokensBuilder) {
    if let Some(identifier) = member.identifier() {
        let syntax = identifier.syntax();
        builder.build_token(syntax.text_range(), *TokenIndex::PROPERTY, *ModifierIndex::NONE);
    }
}
