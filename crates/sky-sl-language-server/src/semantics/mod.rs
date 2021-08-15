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
            ModuleItemKind::ModuleDeclaration(module_declaration) => {
                visit_module_declaration(module_declaration, builder);
            },
        }
    }
}

fn visit_module_declaration(module_declaration: ModuleDeclaration, builder: &mut SemanticTokensBuilder) {
    if let Some(keyword) = module_declaration.syntax().first_token() {
        builder.build_token(keyword.text_range(), *TokenIndex::KEYWORD, *ModifierIndex::NONE);
    }
}

fn visit_function_definition(function_definition: FunctionDefinition, builder: &mut SemanticTokensBuilder) {
    if let Some(signature) = function_definition.signature() {
        if let Some(keyword) = function_definition.syntax().first_token() {
            builder.build_token(keyword.text_range(), *TokenIndex::KEYWORD, *ModifierIndex::NONE);
        }
    
        if let Some(identifier) = signature.identifier() {
            let syntax = identifier.syntax();
            builder.build_token(syntax.text_range(), *TokenIndex::FUNCTION, *ModifierIndex::DECLARATION);
        }
    
        if let Some(argument_list) = signature.argument_list() {
            visit_argument_list(argument_list, builder)
        }
    
        if let Some(return_type) = signature.return_type() {
            if let Some(type_identifier) = return_type.type_identifier() {
                builder.build_token(type_identifier.syntax().text_range(), *TokenIndex::TYPE, *ModifierIndex::NONE);
            }
        }
    }

    if let Some(block) = function_definition.block_definition() {
        visit_block(block, builder);
    }
}

fn visit_argument_list(argument_list: ArgumentList, builder: &mut SemanticTokensBuilder) {
    for argument in argument_list.arguments() {
        if let Some(identifier) = argument.identifier() {
            builder.build_token(identifier.syntax().text_range(), *TokenIndex::PARAMETER, *ModifierIndex::DECLARATION);
        }

        if let Some(type_identifier) = argument.type_identifier() {
            builder.build_token(type_identifier.syntax().text_range(), *TokenIndex::TYPE, *ModifierIndex::NONE);
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

    if let Some(type_identifier) = member.type_identifier() {
        builder.build_token(type_identifier.syntax().text_range(), *TokenIndex::TYPE, *ModifierIndex::NONE);
    }
}

fn visit_block(block: BlockDefinition, builder: &mut SemanticTokensBuilder) {
    for statement in block.statements() {
        visit_statement(statement, builder);
    }
}

fn visit_statement(statement: Statement, builder: &mut SemanticTokensBuilder) {
    match statement.kind() {
        StatementKind::Let(let_statement) => visit_let_statement(let_statement, builder),
        StatementKind::Expression(expression_statement) => visit_expression_statement(expression_statement, builder),
    }
}

fn visit_let_statement(let_statement: LetStatement, builder: &mut SemanticTokensBuilder) {
    if let Some(token) = let_statement.syntax().first_token() {
        builder.build_token(token.text_range(), *TokenIndex::KEYWORD, *ModifierIndex::NONE);
    }

    if let Some(identifier) = let_statement.identifier() {
        builder.build_token(identifier.syntax().text_range(), *TokenIndex::VARIABLE, *ModifierIndex::DEFINITION);
    }

    if let Some(expression) = let_statement.expression() {
        visit_expression(expression, builder);
    }
}

fn visit_expression_statement(expression_statement: ExpressionStatement, builder: &mut SemanticTokensBuilder) {
    if let Some(expression) = expression_statement.expression() {
        visit_expression(expression, builder);
    }
}

fn visit_expression(expression: Expression, builder: &mut SemanticTokensBuilder) {
    match expression.kind() {
        ExpressionKind::LiteralExpression(literal_expression) => visit_literal_expression(literal_expression, builder),
        ExpressionKind::GroupExpression(group_expression) => visit_group_expression(group_expression, builder),
        ExpressionKind::BinaryExpression(binary_expression) => visit_binary_expression(binary_expression, builder),
        ExpressionKind::CallExpression(call_expression) => visit_call_expression(call_expression, builder),
        ExpressionKind::IndexExpression(index_expression) => visit_index_expression(index_expression, builder),
        ExpressionKind::FieldAccessExpression(field_access_expression) => visit_field_access_expression(field_access_expression, builder),
        ExpressionKind::PathExpression(path_expression) => visit_path_expression(path_expression, builder),
        ExpressionKind::StructExpression(struct_expression) => visit_struct_expression(struct_expression, builder),
    }
}

fn visit_literal_expression(literal_expression: LiteralExpression, builder: &mut SemanticTokensBuilder) {
    builder.build_token(literal_expression.syntax().text_range(), *TokenIndex::NUMBER, *ModifierIndex::NONE);
}

fn visit_group_expression(group_expression: GroupExpression, builder: &mut SemanticTokensBuilder) {
    if let Some(child) = group_expression.expression() {
        visit_expression(child, builder);
    }
}

fn visit_binary_expression(binary_expression: BinaryExpression, builder: &mut SemanticTokensBuilder) {
    for child in binary_expression.expressions() {
        visit_expression(child, builder);
    }
}

fn visit_path_expression(path_expression: PathExpression, builder: &mut SemanticTokensBuilder) {
    builder.build_token(path_expression.syntax().text_range(), *TokenIndex::VARIABLE, *ModifierIndex::NONE);
}

fn visit_call_expression(call_expression: CallExpression, builder: &mut SemanticTokensBuilder) {
    if let Some(child) = call_expression.expression() {
        visit_expression(child, builder);
    }

    if let Some(arguments) = call_expression.arguments() {
        for child in arguments.expressions() {
            visit_expression(child, builder);
        }
    }
}

fn visit_index_expression(index_expression: IndexExpression, builder: &mut SemanticTokensBuilder) {
    for child in index_expression.expressions() {
        visit_expression(child, builder);
    }
}

fn visit_field_access_expression(field_access_expression: FieldAccessExpression, builder: &mut SemanticTokensBuilder) {
    for child in field_access_expression.expressions() {
        visit_expression(child, builder);
    }
}

fn visit_struct_expression(struct_expression: StructExpression, builder: &mut SemanticTokensBuilder) {

    if let Some(expr) = struct_expression.expression() {
        // TODO clean up AST
        builder.build_token(expr.syntax().text_range(), *TokenIndex::STRUCT, *ModifierIndex::NONE);
    }

    if let Some(fields) = struct_expression.fields() {
        for field in fields.fields() {
            if let Some(identifier) = field.identifier() {
                builder.build_token(identifier.syntax().text_range(), *TokenIndex::PROPERTY, *ModifierIndex::NONE);
            }

            if let Some(expression) = field.expression() {
                visit_expression(expression, builder);
            }
        }
    }
}
