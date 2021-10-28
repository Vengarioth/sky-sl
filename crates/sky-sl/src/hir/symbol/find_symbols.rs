use crate::{fs::FileId, hir::HirDatabase, syn::ast::*};

use super::{Symbol, SymbolKind, SymbolList, SymbolMember, SymbolMemberKind, builder::SymbolListBuilder};

pub fn find_symbols(file_id: FileId, ast: Root, db: &dyn HirDatabase) -> SymbolList {
    let mut builder = SymbolListBuilder::new(file_id, db);

    for item in ast.module_items() {
        match item.kind() {
            ModuleItemKind::ModuleDeclaration(module_declaration) => {
                visit_module_declaration(module_declaration, &mut builder)
            }
            ModuleItemKind::FunctionDefinition(function_definition) => {
                visit_function_definition(function_definition, &mut builder)
            }
            ModuleItemKind::StructDefinition(struct_definition) => {
                visit_struct_definition(struct_definition, &mut builder)
            }
            ModuleItemKind::UseDeclaration(_use_declaration) => {}
            ModuleItemKind::LayoutDefinition(layout_definition) => {
                visit_layout_definition(layout_definition, &mut builder)
            }
        }
    }

    builder.build()
}

fn visit_module_declaration(
    module_declaration: ModuleDeclaration,
    builder: &mut SymbolListBuilder,
) {
    if let Some(name_syntax) = module_declaration.name() {
        let name = builder.intern_name(name_syntax.syntax.to_string());
        let span = module_declaration.syntax.text_range();
        let selection_span = name_syntax.syntax.text_range();

        builder.add_symbol(Symbol::new(
            name,
            builder.current_file(),
            span,
            selection_span,
            SymbolKind::Module,
            Vec::new(),
        ));
    }
}

fn visit_function_definition(
    function_definition: FunctionDefinition,
    builder: &mut SymbolListBuilder,
) {
    if let Some(function_name) = function_definition.signature().and_then(|s| s.name()) {
        let name = builder.intern_name(function_name.syntax.to_string());
        let span = function_definition.syntax.text_range();
        let selection_span = function_name.syntax.text_range();
        builder.add_symbol(Symbol::new(
            name,
            builder.current_file(),
            span,
            selection_span,
            SymbolKind::Function,
            Vec::new(),
        ));
    }
}

fn visit_struct_definition(
    struct_definition: StructDefinition,
    builder: &mut SymbolListBuilder,
) {
    if let Some(struct_name) = struct_definition.name() {
        let name = builder.intern_name(struct_name.syntax.to_string());
        let span = struct_definition.syntax.text_range();
        let selection_span = struct_name.syntax.text_range();

        let mut member_list = Vec::new();
        if let Some(m) = struct_definition.member_list() {
            for member in m.member() {
                visit_struct_member(member, builder, &mut member_list);
            }
        }

        builder.add_symbol(Symbol::new(
            name,
            builder.current_file(),
            span,
            selection_span,
            SymbolKind::Struct,
            member_list,
        ));
    }
}

fn visit_struct_member(
    member: Member,
    builder: &mut SymbolListBuilder,
    member_list: &mut Vec<SymbolMember>,
) {
    if let Some(member_name) = member.name() {
        let name = builder.intern_name(member_name.syntax.to_string());
        let span = member.syntax().text_range();
        let selection_span = member_name.syntax.text_range();

        member_list.push(SymbolMember::new(
            name,
            span,
            selection_span,
            SymbolMemberKind::Field,
        ));
    }
}

fn visit_layout_definition(
    layout_definition: LayoutDefinition,
    builder: &mut SymbolListBuilder,
) {
    if let Some(struct_name) = layout_definition.name() {
        let name = builder.intern_name(struct_name.syntax.to_string());
        let span = layout_definition.syntax.text_range();
        let selection_span = struct_name.syntax.text_range();

        let mut member_list = Vec::new();
        if let Some(m) = layout_definition.layout_member_list() {
            for member in m.layout_member() {
                visit_layout_member(member, builder, &mut member_list);
            }
        }

        builder.add_symbol(Symbol::new(
            name,
            builder.current_file(),
            span,
            selection_span,
            SymbolKind::Layout,
            member_list,
        ));
    }
}

fn visit_layout_member(
    member: LayoutMember,
    builder: &mut SymbolListBuilder,
    member_list: &mut Vec<SymbolMember>,
) {
    if let Some(member_name) = member.name() {
        let name = builder.intern_name(member_name.syntax.to_string());
        let span = member.syntax().text_range();
        let selection_span = member_name.syntax.text_range();

        member_list.push(SymbolMember::new(
            name,
            span,
            selection_span,
            SymbolMemberKind::Field,
        ));
    }
}
