use super::HirDatabase;
use crate::fs::FileId;
use crate::syn::ast::*;
use crate::hir::untyped;

mod builder;
mod error;
use builder::*;
pub use error::*;

pub fn lower_ast_to_hir(file: FileId, db: &dyn HirDatabase, ast: &Root) -> untyped::Module {
    let mut builder = HirModuleBuilder::new(db, file);
    for use_declaration in ast.uses() {
        visit_use_declaration(use_declaration, &mut builder);
    }

    for module in ast.module_declarations() {
        if let Some(x) = module.name() {
            let module_file_name = format!("{}.skysl", x.syntax());

            let parent = db.directory(file);
            let _module_file = db.child_file(parent, module_file_name);
        }
    }

    for item in ast.module_items() {
        match item.kind() {
            ModuleItemKind::FunctionDefinition(function_definition) => visit_function_definition(function_definition, &mut builder),
            ModuleItemKind::StructDefinition(struct_definition) => visit_struct_definition(struct_definition, &mut builder),
            ModuleItemKind::LayoutDefinition(layout_definition) => visit_layout_definition(layout_definition, &mut builder),
            _ => { /* TODO */ }
        };
    }

    builder.build()
}

fn visit_use_declaration(use_declaration: UseDeclaration, builder: &mut HirModuleBuilder) {
    lower_use_declaration(use_declaration, builder).unwrap();
}

fn lower_use_declaration(use_declaration: UseDeclaration, builder: &mut HirModuleBuilder) -> Result<(), LowerToHirError> {
    let use_tree = use_declaration.use_tree().ok_or_else(|| LowerToHirError::IncompleteUseDeclaration)?;

    Ok(())
}

fn visit_function_definition(function_definition: FunctionDefinition, builder: &mut HirModuleBuilder) {
    match lower_function_definition(function_definition, builder) {
        Ok(function_kind) => builder.add_item(untyped::ItemKind::Function(function_kind)),
        Err(error) => builder.add_diagnostic(error),
    }
}

fn lower_function_definition(function_definition: FunctionDefinition, builder: &mut HirModuleBuilder) -> Result<untyped::FunctionKind, LowerToHirError> {
    let function_signature = function_definition.signature().map(|s| lower_function_signature(s, builder)).ok_or_else(|| LowerToHirError::MissingFunctionSignature)??;
    let block = lower_block_definition(function_definition.block_definition().ok_or_else(|| LowerToHirError::IncompleteFunctionBody)?, builder)?;
    Ok(untyped::FunctionKind::new(function_signature, block, function_definition.syntax.text_range()))
}

fn lower_function_signature(function_signature: FunctionSignature, builder: &mut HirModuleBuilder) -> Result<untyped::FunctionSignature, LowerToHirError> {
    let name = function_signature.name().ok_or_else(|| LowerToHirError::IncompleteFunctionSignature)?;
    let name = builder.intern_name(name.syntax().to_string());
    let item = builder.lookup_item(name).ok_or_else(|| LowerToHirError::IncompleteStructDeclaration)?;

    let mut arguments = Vec::new();
    let argument_list = function_signature.argument_list().ok_or_else(|| LowerToHirError::IncompleteFunctionSignature)?;
    for argument in argument_list.arguments() {
        let name = argument.name().ok_or_else(|| LowerToHirError::IncompleteFunctionSignature)?;
        let name = builder.intern_name(name.syntax().to_string());
        let ty_name = argument.path().ok_or_else(|| LowerToHirError::IncompleteFunctionSignature)?;
        let ty_name = builder.intern_name(ty_name.syntax().to_string());
        let item_type = builder.lookup_item(ty_name).ok_or_else(|| LowerToHirError::IncompleteStructDeclaration)?;

        arguments.push(untyped::FunctionArgument::new(
            name,
            item_type,
            argument.syntax().text_range(), 
        ));
    }

    let return_type = if let Some(return_type) = function_signature.return_type() {
        let return_type_name = return_type.path().ok_or_else(|| LowerToHirError::IncompleteFunctionSignature)?;
        let return_type_name = builder.intern_name(return_type_name.syntax().to_string());
        let return_type = builder.lookup_item(return_type_name).ok_or_else(|| LowerToHirError::IncompleteStructDeclaration)?;

        Some(return_type)
    } else {
        None
    };

    Ok(untyped::FunctionSignature::new(
        name,
        item,
        arguments,
        return_type,
        function_signature.syntax().text_range()
    ))
}

fn lower_block_definition(block_definition: BlockDefinition, builder: &mut HirModuleBuilder) -> Result<untyped::Block, LowerToHirError> {

    let mut statements = Vec::new();
    for statement in block_definition.statements() {
        statements.push(lower_statement(statement, builder)?);
    }

    Ok(untyped::Block::new(
        statements,
        block_definition.syntax().text_range()
    ))
}

fn lower_statement(statement: Statement, builder: &mut HirModuleBuilder) -> Result<untyped::StatementKind, LowerToHirError> {
    match statement.kind() {
        StatementKind::Let(let_statement) => Ok(untyped::StatementKind::Let(lower_let_statement(let_statement, builder)?)),
        StatementKind::Expression(expression_statement) => Ok(untyped::StatementKind::Expression(lower_expression_statement(expression_statement, builder)?)),
    }
}

fn lower_let_statement(let_statement: LetStatement, builder: &mut HirModuleBuilder) -> Result<untyped::LetStatement, LowerToHirError> {
    let name = let_statement.identifier().ok_or_else(|| LowerToHirError::IncompleteStatement)?;
    let name = builder.intern_name(name.syntax().to_string());
    let expression = lower_expression(let_statement.expression().ok_or_else(|| LowerToHirError::IncompleteStatement)?, builder)?;
    
    Ok(untyped::LetStatement::new(
        name,
        expression,
        let_statement.syntax().text_range()
    ))
}

fn lower_expression_statement(expression_statement: ExpressionStatement, builder: &mut HirModuleBuilder) -> Result<untyped::ExpressionStatement, LowerToHirError> {
    let expression = lower_expression(expression_statement.expression().ok_or_else(|| LowerToHirError::IncompleteStatement)?, builder)?;
    
    Ok(untyped::ExpressionStatement::new(
        expression,
        expression_statement.syntax().text_range()
    ))
}

fn lower_expression(expression: Expression, builder: &mut HirModuleBuilder) -> Result<untyped::ExpressionKind, LowerToHirError> {
    match expression.kind() {
        ExpressionKind::LiteralExpression(litreal_expression) => Ok(untyped::ExpressionKind::LiteralExpression(lower_literal_expression(litreal_expression)?)),
        ExpressionKind::GroupExpression(group_expression) => Ok(untyped::ExpressionKind::GroupExpression(lower_group_expression(group_expression, builder)?)),
        ExpressionKind::BinaryExpression(binary_expression) => Ok(untyped::ExpressionKind::BinaryExpression(lower_binary_expression(binary_expression, builder)?)),
        ExpressionKind::CallExpression(call_expression) => Ok(untyped::ExpressionKind::CallExpression(lower_call_expression(call_expression, builder)?)),
        ExpressionKind::FieldAccessExpression(field_access_expression) => Ok(untyped::ExpressionKind::FieldAccessExpression(lower_field_access_expression(field_access_expression)?)),
        ExpressionKind::IndexExpression(index_expression) => Ok(untyped::ExpressionKind::IndexExpression(lower_index_expression(index_expression)?)),
        ExpressionKind::PathExpression(path_expression) => Ok(untyped::ExpressionKind::PathExpression(lower_path_expression(path_expression, builder)?)),
        ExpressionKind::StructExpression(struct_expression) => Ok(untyped::ExpressionKind::StructExpression(lower_struct_expression(struct_expression)?)),
    }
}

fn lower_literal_expression(literal_expression: LiteralExpression) -> Result<untyped::LiteralExpression, LowerToHirError> {
    Ok(untyped::LiteralExpression::new(literal_expression.syntax().text_range()))
}

fn lower_group_expression(group_expression: GroupExpression, builder: &mut HirModuleBuilder) -> Result<untyped::GroupExpression, LowerToHirError> {
    let inner = lower_expression(group_expression.expression().ok_or_else(|| LowerToHirError::IncompleteExpression)?, builder)?;
    Ok(untyped::GroupExpression::new(Box::new(inner), group_expression.syntax().text_range()))
}

fn lower_binary_expression(binary_expression: BinaryExpression, builder: &mut HirModuleBuilder) -> Result<untyped::BinaryExpression, LowerToHirError> {
    let lhs = lower_expression(binary_expression.lhs().ok_or_else(|| LowerToHirError::IncompleteExpression)?, builder)?;
    let rhs = lower_expression(binary_expression.rhs().ok_or_else(|| LowerToHirError::IncompleteExpression)?, builder)?;
    let _operator = binary_expression.operator().ok_or_else(|| LowerToHirError::IncompleteExpression)?;

    // TODO
    // dbg!(operator);

    Ok(untyped::BinaryExpression::new(
        Box::new(lhs),
        Box::new(rhs),
        binary_expression.syntax().text_range(),
    ))
}

fn lower_call_expression(call_expression: CallExpression, builder: &mut HirModuleBuilder) -> Result<untyped::CallExpression, LowerToHirError> {
    let mut arguments = Vec::new();
    let argument_list = call_expression.arguments().ok_or_else(|| LowerToHirError::IncompleteExpression)?;

    for argument_expr in argument_list.expressions() {
        arguments.push(lower_expression(argument_expr, builder)?);
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

fn lower_path_expression(path_expression: PathExpression, builder: &mut HirModuleBuilder) -> Result<untyped::PathExpression, LowerToHirError> {
    // TODO
    let name = builder.intern_name(path_expression.syntax().to_string());
    Ok(untyped::PathExpression::new(
        name,
        path_expression.syntax().text_range(),
    ))
}

fn lower_struct_expression(struct_expression: StructExpression) -> Result<untyped::StructExpression, LowerToHirError> {
    // TODO
    Ok(untyped::StructExpression::new(
        struct_expression.syntax().text_range(),
    ))
}

fn visit_struct_definition(struct_definition: StructDefinition, builder: &mut HirModuleBuilder) {
    match lower_struct_definition(struct_definition, builder) {
        Ok(struct_kind) => builder.add_item(untyped::ItemKind::Struct(struct_kind)),
        Err(diagnostic) => builder.add_diagnostic(diagnostic),
    }
}

fn lower_struct_definition(struct_definition: StructDefinition, builder: &mut HirModuleBuilder) -> Result<untyped::StructKind, LowerToHirError> {
    let name = struct_definition.name().ok_or_else(|| LowerToHirError::IncompleteStructDeclaration)?;
    let name = builder.intern_name(name.syntax().to_string());
    let item = builder.lookup_item(name).ok_or_else(|| LowerToHirError::IncompleteStructDeclaration)?;

    let mut members = Vec::new();
    let member_list = struct_definition.member_list().ok_or_else(|| LowerToHirError::IncompleteStructDeclaration)?;
    for member in member_list.member() {
        let name = member.name().ok_or_else(|| LowerToHirError::IncompleteStructDeclaration)?;
        let name = builder.intern_name(name.syntax().to_string());

        let item_path = member.path().ok_or_else(|| LowerToHirError::IncompleteStructDeclaration)?;
        let item_path = lower_path(item_path, builder)?;

        members.push(untyped::StructMember::new(
            name,
            item_path,
            member.syntax().text_range(),
        ));
    }

    Ok(untyped::StructKind::new(
        name,
        item,
        members,
        struct_definition.syntax.text_range()
    ))
}

fn visit_layout_definition(layout_definition: LayoutDefinition, builder: &mut HirModuleBuilder) {
    match lower_layout_definition(layout_definition, builder) {
        Ok(layout_kind) => builder.add_item(untyped::ItemKind::Layout(layout_kind)),
        Err(error) => builder.add_diagnostic(error),
    }
}

fn lower_layout_definition(layout_definition: LayoutDefinition, builder: &mut HirModuleBuilder) -> Result<untyped::LayoutKind, LowerToHirError> {
    let name = layout_definition.name().ok_or_else(|| LowerToHirError::IncompleteLayoutDeclaration)?;
    let name = builder.intern_name(name.syntax().to_string());
    let item = builder.lookup_item(name).ok_or_else(|| LowerToHirError::IncompleteLayoutDeclaration)?;

    let mut members = Vec::new();

    let member_list = layout_definition.layout_member_list().ok_or_else(|| LowerToHirError::IncompleteLayoutDeclaration)?;
    for member in member_list.layout_member() {
        let name = member.name().ok_or_else(|| LowerToHirError::IncompleteLayoutDeclaration)?;
        let name = builder.intern_name(name.syntax().to_string());
        let item_path = member.path().ok_or_else(|| LowerToHirError::IncompleteLayoutDeclaration)?;
        let item_path = lower_path(item_path, builder)?;

        members.push(untyped::LayoutMember::new(
            name,
            item_path,
            member.syntax().text_range(), 
        ));
    }

    Ok(untyped::LayoutKind::new(
        name,
        item,
        members,
        layout_definition.syntax.text_range()
    ))
}

fn lower_path(path: Path, builder: &mut HirModuleBuilder) -> Result<untyped::ItemPath, LowerToHirError> {
    let first_segment = path.segment().ok_or_else(|| LowerToHirError::IncompletePath)?;
    let first_segment = lower_path_segment(first_segment, builder, builder.current_file(), true)?;

    Ok(untyped::ItemPath::new(
        first_segment,
        path.syntax().text_range(),
    ))
}

fn lower_path_segment(path_segment: PathSegment, builder: &mut HirModuleBuilder, scope: FileId, local: bool) -> Result<untyped::ItemPathSegment, LowerToHirError> {
    let name = path_segment.name().ok_or_else(|| LowerToHirError::IncompletePath)?;
    let name = builder.intern_name(name.syntax().to_string());

    let item = if local {
        builder.lookup_item(name).ok_or_else(|| LowerToHirError::IncompletePath)?
    } else {
        builder.lookup_item_in(name, scope).ok_or_else(|| LowerToHirError::IncompletePath)?
    };

    let next_segment = if let Some(next) = path_segment.segment() {
        let next_scope = builder.lookup_module_file(scope, name).ok_or_else(|| LowerToHirError::IncompletePath)?;
        Some(Box::new(lower_path_segment(next, builder, next_scope, false)?))
    } else {
        None
    };

    Ok(untyped::ItemPathSegment::new(
        name,
        item,
        next_segment,
        path_segment.syntax().text_range(),
    ))
}
