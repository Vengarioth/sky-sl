use crate::{fs::FileId, syn::ast::*};

use super::{Symbol, SymbolKind, SymbolList};

pub fn find_symbols(file_id: FileId, ast: Root) -> SymbolList {
    let mut symbols = Vec::new();
    dbg!(&ast);

    for item in ast.module_items() {
        match item.kind() {
            ModuleItemKind::ModuleDeclaration(module_declaration) => {
                visit_module_declaration(file_id, module_declaration, &mut symbols)
            },
            ModuleItemKind::FunctionDefinition(function_definition) => {
                visit_function_definition(file_id, function_definition, &mut symbols)
            },
            ModuleItemKind::StructDefinition(struct_definition) => {
                visit_struct_definition(file_id, struct_definition, &mut symbols)
            },
            ModuleItemKind::UseDeclaration(_use_declaration) => {},
        }
    }

    SymbolList::new(symbols)
}

fn visit_module_declaration(
    file_id: FileId,
    module_declaration: ModuleDeclaration,
    symbols: &mut Vec<Symbol>,
) {
    if let Some(module_identifier) = module_declaration.identifier() {
        let name = module_identifier.syntax.to_string();
        let span = module_declaration.syntax.text_range();
        let selection_span = module_identifier.syntax.text_range();

        symbols.push(Symbol::new(
            name,
            file_id,
            span,
            selection_span,
            SymbolKind::Module,
            None,
        ));
    }
}

fn visit_function_definition(
    file_id: FileId,
    function_definition: FunctionDefinition,
    symbols: &mut Vec<Symbol>,
) {
    if let Some(function_identifier) = function_definition.signature().and_then(|s| s.identifier())
    {
        let name = function_identifier.syntax.to_string();
        let span = function_definition.syntax.text_range();
        let selection_span = function_identifier.syntax.text_range();
        symbols.push(Symbol::new(
            name,
            file_id,
            span,
            selection_span,
            SymbolKind::Function,
            None,
        ));
    }
}

fn visit_struct_definition(
    file_id: FileId,
    struct_definition: StructDefinition,
    symbols: &mut Vec<Symbol>,
) {
    if let Some(struct_identifier) = struct_definition.identifier() {
        let name = struct_identifier.syntax.to_string();
        let span = struct_definition.syntax.text_range();
        let selection_span = struct_identifier.syntax.text_range();

        let mut children = Vec::new();

        if let Some(member_list) = struct_definition.member_list() {
            for member in member_list.member() {
                visit_struct_member(file_id, member, &mut children);
            }
        }

        symbols.push(Symbol::new(
            name,
            file_id,
            span,
            selection_span,
            SymbolKind::Struct,
            Some(children),
        ));
    }
}

fn visit_struct_member(file_id: FileId, member: Member, symbols: &mut Vec<Symbol>) {
    if let Some(identifier) = member.identifier() {
        let name = identifier.syntax.to_string();
        let span = member.syntax().text_range();
        let selection_span = identifier.syntax.text_range();
        symbols.push(Symbol::new(
            name,
            file_id,
            span,
            selection_span,
            SymbolKind::Field,
            None,
        ));
    }
}
