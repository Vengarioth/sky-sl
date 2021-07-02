use crate::syn::{ast::*};
use naga::*;

pub fn lower_ast(root: Root) {
    visit_root(root);
}

fn visit_root(root: Root) {
    for module_item in root.module_items() {
        match module_item.kind() {
            ModuleItemKind::FunctionDefinition(_) => todo!(),
            ModuleItemKind::StructDefinition(struct_definition) => visit_struct_definition(struct_definition),
        }
    }
}

fn visit_struct_definition(struct_definition: StructDefinition) {
    let mut arena = Arena::new();

    let dummy = arena.append(naga::Type {
        name: None,
        inner: TypeInner::Scalar {
            kind: ScalarKind::Float,
            width: 4,
        },
    });

    let mut members = Vec::new();
    if let Some(member_list) = struct_definition.member_list() {
        for member in member_list.member() {
            members.push(StructMember {
                name: member.identifier().map(|identifier| identifier.syntax().to_string()),
                binding: None, // TODO
                offset: 0, // TODO
                ty: dummy, // TODO
            });
        }
    }

    
    let _struct_type = arena.append(naga::Type {
        name: struct_definition.identifier().map(|identifier| identifier.syntax().to_string()),
        inner: TypeInner::Struct {
            top_level: true,
            members,
            span: 0,
        },
    });
}
