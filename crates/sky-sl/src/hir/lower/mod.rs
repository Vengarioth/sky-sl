use crate::syn::ast::*;
use crate::hir::untyped;

mod error;

pub use error::*;

pub fn lower_ast_to_hir(ast: &Root) -> untyped::Module {
    
    let mut items = Vec::new();
    let mut errors = Vec::new();
    for item in ast.module_items() {
        match item.kind() {
            ModuleItemKind::FunctionDefinition(function_definition) => visit_function_definition(function_definition, &mut items, &mut errors),
            ModuleItemKind::StructDefinition(struct_definition) => visit_struct_definition(struct_definition, &mut items, &mut errors),
            _ => { /* TODO */ }
        };
    }

    untyped::Module::new(items, errors, ast.syntax().text_range())
}

fn visit_function_definition(function_definition: FunctionDefinition, items: &mut Vec<untyped::ItemKind>, errors: &mut Vec<LowerToHirError>) {
    match lower_function_definition(function_definition) {
        Ok(function_kind) => items.push(untyped::ItemKind::Function(function_kind)),
        Err(error) => errors.push(error),
    }
}

fn lower_function_definition(function_definition: FunctionDefinition) -> Result<untyped::FunctionKind, LowerToHirError> {
    let function_signature = function_definition.signature().map(|s| lower_function_signature(s)).ok_or_else(|| LowerToHirError::MissingFunctionSignature)??;
    let block = lower_block_definition(function_definition.block_definition().ok_or_else(|| LowerToHirError::IncompleteFunctionBody)?)?;
    Ok(untyped::FunctionKind::new(function_signature, block, function_definition.syntax.text_range()))
}

fn lower_function_signature(function_signature: FunctionSignature) -> Result<untyped::FunctionSignature, LowerToHirError> {
    let name = function_signature.identifier().ok_or_else(|| LowerToHirError::IncompleteFunctionSignature)?;
    let mut arguments = Vec::new();

    let argument_list = function_signature.argument_list().ok_or_else(|| LowerToHirError::IncompleteFunctionSignature)?;
    for argument in argument_list.arguments() {
        let name = argument.identifier().ok_or_else(|| LowerToHirError::IncompleteFunctionSignature)?;
        let ty_name = argument.type_identifier().ok_or_else(|| LowerToHirError::IncompleteFunctionSignature)?;
        arguments.push(untyped::FunctionArgument::new(
            name.syntax().to_string(),
            ty_name.syntax().to_string(),
            argument.syntax().text_range(), 
        ));
    }

    let return_type = if let Some(return_type) = function_signature.return_type() {
        let type_identifier = return_type.type_identifier().ok_or_else(|| LowerToHirError::IncompleteFunctionSignature)?;
        Some(type_identifier.syntax().to_string())
    } else {
        None
    };

    Ok(untyped::FunctionSignature::new(
        name.syntax().to_string(),
        arguments,
        return_type,
        function_signature.syntax().text_range()
    ))
}

fn lower_block_definition(block_definition: BlockDefinition) -> Result<untyped::Block, LowerToHirError> {

    let mut statements = Vec::new();
    for statement in block_definition.statements() {
        statements.push(lower_statement(statement)?);
    }

    Ok(untyped::Block::new(
        statements,
        block_definition.syntax().text_range()
    ))
}

fn lower_statement(statement: Statement) -> Result<untyped::StatementKind, LowerToHirError> {
    match statement.kind() {
        StatementKind::Let(let_statement) => Ok(untyped::StatementKind::Let(lower_let_statement(let_statement)?)),
        StatementKind::Expression(expression_statement) => Ok(untyped::StatementKind::Expression(lower_expression_statement(expression_statement)?)),
    }
}

fn lower_let_statement(let_statement: LetStatement) -> Result<untyped::LetStatement, LowerToHirError> {
    let name = let_statement.identifier().ok_or_else(|| LowerToHirError::IncompleteStatement)?;
    let expression = lower_expression(let_statement.expression().ok_or_else(|| LowerToHirError::IncompleteStatement)?)?;
    
    Ok(untyped::LetStatement::new(
        name.syntax().to_string(),
        expression,
        let_statement.syntax().text_range()
    ))
}

fn lower_expression_statement(expression_statement: ExpressionStatement) -> Result<untyped::ExpressionStatement, LowerToHirError> {
    let expression = lower_expression(expression_statement.expression().ok_or_else(|| LowerToHirError::IncompleteStatement)?)?;
    
    Ok(untyped::ExpressionStatement::new(
        expression,
        expression_statement.syntax().text_range()
    ))
}

fn lower_expression(expression: Expression) -> Result<untyped::ExpressionKind, LowerToHirError> {
    match expression.kind() {
        ExpressionKind::LiteralExpression(litreal_expression) => Ok(untyped::ExpressionKind::LiteralExpression(lower_literal_expression(litreal_expression)?)),
        ExpressionKind::GroupExpression(group_expression) => Ok(untyped::ExpressionKind::GroupExpression(lower_group_expression(group_expression)?)),
        ExpressionKind::BinaryExpression(binary_expression) => Ok(untyped::ExpressionKind::BinaryExpression(lower_binary_expression(binary_expression)?)),
        ExpressionKind::CallExpression(call_expression) => Ok(untyped::ExpressionKind::CallExpression(lower_call_expression(call_expression)?)),
        ExpressionKind::FieldAccessExpression(field_access_expression) => Ok(untyped::ExpressionKind::FieldAccessExpression(lower_field_access_expression(field_access_expression)?)),
        ExpressionKind::IndexExpression(index_expression) => Ok(untyped::ExpressionKind::IndexExpression(lower_index_expression(index_expression)?)),
        ExpressionKind::PathExpression(path_expression) => Ok(untyped::ExpressionKind::PathExpression(lower_path_expression(path_expression)?)),
        ExpressionKind::StructExpression(struct_expression) => Ok(untyped::ExpressionKind::StructExpression(lower_struct_expression(struct_expression)?)),
    }
}

fn lower_literal_expression(literal_expression: LiteralExpression) -> Result<untyped::LiteralExpression, LowerToHirError> {
    Ok(untyped::LiteralExpression::new(literal_expression.syntax().text_range()))
}

fn lower_group_expression(group_expression: GroupExpression) -> Result<untyped::GroupExpression, LowerToHirError> {
    let inner = lower_expression(group_expression.expression().ok_or_else(|| LowerToHirError::IncompleteExpression)?)?;
    Ok(untyped::GroupExpression::new(Box::new(inner), group_expression.syntax().text_range()))
}

fn lower_binary_expression(binary_expression: BinaryExpression) -> Result<untyped::BinaryExpression, LowerToHirError> {
    let lhs = lower_expression(binary_expression.lhs().ok_or_else(|| LowerToHirError::IncompleteExpression)?)?;
    let rhs = lower_expression(binary_expression.rhs().ok_or_else(|| LowerToHirError::IncompleteExpression)?)?;
    let operator = binary_expression.operator().ok_or_else(|| LowerToHirError::IncompleteExpression)?;

    // TODO
    dbg!(operator);

    Ok(untyped::BinaryExpression::new(
        Box::new(lhs),
        Box::new(rhs),
        binary_expression.syntax().text_range(),
    ))
}

fn lower_call_expression(call_expression: CallExpression) -> Result<untyped::CallExpression, LowerToHirError> {
    let mut arguments = Vec::new();
    let argument_list = call_expression.arguments().ok_or_else(|| LowerToHirError::IncompleteExpression)?;

    for argument_expr in argument_list.expressions() {
        arguments.push(lower_expression(argument_expr)?);
    }

    Ok(untyped::CallExpression::new(
        arguments,
        call_expression.syntax().text_range(),
    ))
}

fn lower_field_access_expression(field_access_expression: FieldAccessExpression) -> Result<untyped::FieldAccessExpression, LowerToHirError> {
    // TODO
    Ok(untyped::FieldAccessExpression::new(
        field_access_expression.syntax().text_range(),
    ))
}

fn lower_index_expression(index_expression: IndexExpression) -> Result<untyped::IndexExpression, LowerToHirError> {
    // TODO
    Ok(untyped::IndexExpression::new(
        index_expression.syntax().text_range(),
    ))
}

fn lower_path_expression(path_expression: PathExpression) -> Result<untyped::PathExpression, LowerToHirError> {
    // TODO
    Ok(untyped::PathExpression::new(
        path_expression.syntax().to_string(),
        path_expression.syntax().text_range(),
    ))
}

fn lower_struct_expression(struct_expression: StructExpression) -> Result<untyped::StructExpression, LowerToHirError> {
    // TODO
    Ok(untyped::StructExpression::new(
        struct_expression.syntax().text_range(),
    ))
}

fn visit_struct_definition(struct_definition: StructDefinition, items: &mut Vec<untyped::ItemKind>, errors: &mut Vec<LowerToHirError>) {
    match lower_struct_definition(struct_definition) {
        Ok(struct_kind) => items.push(untyped::ItemKind::Struct(struct_kind)),
        Err(error) => errors.push(error),
    }
}

fn lower_struct_definition(struct_definition: StructDefinition) -> Result<untyped::StructKind, LowerToHirError> {
    let name = struct_definition.identifier().ok_or_else(|| LowerToHirError::IncompleteStructDeclaration)?;

    let mut members = Vec::new();

    let member_list = struct_definition.member_list().ok_or_else(|| LowerToHirError::IncompleteStructDeclaration)?;
    for member in member_list.member() {
        let name = member.identifier().ok_or_else(|| LowerToHirError::IncompleteStructDeclaration)?;
        let ty_name = member.type_identifier().ok_or_else(|| LowerToHirError::IncompleteStructDeclaration)?;

        members.push(untyped::StructMember::new(
            name.syntax().to_string(),
            ty_name.syntax().to_string(),
            member.syntax().text_range(), 
        ));
    }

    Ok(untyped::StructKind::new(
        name.syntax().to_string(),
        members,
        struct_definition.syntax.text_range()
    ))
}

#[cfg(test)]
mod tests {
    use crate::db::*;
    use camino::Utf8PathBuf;
    use std::str::FromStr;
    use std::sync::Arc;

    #[test]
    fn it_works() {
        let mut db = CompilerDatabase::default();

        let path = Utf8PathBuf::from_str("/foo/bar").unwrap();
        let input = "fn bar(x: Y) -> Bar {} fn foo(a: B) { let a = 1 + 2; } struct Bar { foo: Baz, }".to_string();
        db.set_input_file(path.clone(), Arc::from(input));

        let ast = db.ast(path.clone());
        let hir = db.hir(path);

        dbg!(ast.errors(), ast.tree(), hir);
    }
}
