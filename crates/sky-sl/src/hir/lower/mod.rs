use crate::syn::ast::*;
use crate::hir;

mod error;

pub use error::*;

pub fn lower_ast_to_hir(ast: &Root) -> hir::Module {
    
    let mut items = Vec::new();
    let mut errors = Vec::new();
    for item in ast.module_items() {
        match item.kind() {
            ModuleItemKind::FunctionDefinition(function_definition) => visit_function_definition(function_definition, &mut items, &mut errors),
            ModuleItemKind::StructDefinition(struct_definition) => visit_struct_definition(struct_definition, &mut items, &mut errors),
        };
    }

    hir::Module::new(items, errors, ast.syntax().text_range())
}

fn visit_function_definition(function_definition: FunctionDefinition, items: &mut Vec<hir::ItemKind>, errors: &mut Vec<LowerToHirError>) {
    match lower_function_definition(function_definition) {
        Ok(function_kind) => items.push(hir::ItemKind::Function(function_kind)),
        Err(error) => errors.push(error),
    }
}

fn lower_function_definition(function_definition: FunctionDefinition) -> Result<hir::FunctionKind, LowerToHirError> {
    let function_signature = function_definition.signature().map(|s| lower_function_signature(s)).ok_or_else(|| LowerToHirError::MissingFunctionSignature)??;
    Ok(hir::FunctionKind::new(function_signature, function_definition.syntax.text_range()))
}

fn lower_function_signature(function_signature: FunctionSignature) -> Result<hir::FunctionSignature, LowerToHirError> {
    let mut arguments = Vec::new();

    let argument_list = function_signature.argument_list().ok_or_else(|| LowerToHirError::IncompleteFunctionSignature)?;
    for argument in argument_list.arguments() {
        let name = argument.identifier().ok_or_else(|| LowerToHirError::IncompleteFunctionSignature)?;
        let ty_name = argument.type_identifier().ok_or_else(|| LowerToHirError::IncompleteFunctionSignature)?;
        arguments.push(hir::FunctionArgument::new(
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

    Ok(hir::FunctionSignature::new(arguments, return_type, function_signature.syntax().text_range()))
}

fn visit_struct_definition(struct_definition: StructDefinition, items: &mut Vec<hir::ItemKind>, _errors: &mut Vec<LowerToHirError>) {
    // TODO
    items.push(hir::ItemKind::Structure(hir::StructureKind::new(struct_definition.syntax.text_range())));
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
        let input = "fn bar(x: Y) -> Bar {} fn foo(a: B) {}".to_string();
        db.set_input_file(path.clone(), Arc::from(input));

        let ast = db.ast(path.clone());
        let hir = db.hir(path);

        dbg!(ast.errors(), ast.tree(), hir);
    }
}
