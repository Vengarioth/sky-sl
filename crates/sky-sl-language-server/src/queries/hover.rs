use sky_sl::{
    hir::{primitive::Primitive, symbol::Symbol, untyped::*},
    syn::cst::LineIndex,
    text::Locate,
    workspace::Workspace,
};
use tower_lsp::lsp_types::*;

pub fn hover(
    hir: Module,
    position: Position,
    line_index: LineIndex,
    workspace: &Workspace,
) -> Option<Hover> {
    let offset = line_index.find_offset(position.line, position.character);

    match hir.locate(offset)? {
        sky_sl::hir::named::NamedItemKind::Symbol(symbol) => Some(hover_symbol(&symbol, workspace)),
        sky_sl::hir::named::NamedItemKind::Primitive(primitive) => {
            Some(hover_primitive(&primitive, workspace))
        }
    }
}

fn hover_symbol(symbol: &Symbol, workspace: &Workspace) -> Hover {
    let name = workspace.interned_name(symbol.name);

    let kind = match symbol.kind {
        sky_sl::hir::symbol::SymbolKind::Module => "module",
        sky_sl::hir::symbol::SymbolKind::Function => "function",
        sky_sl::hir::symbol::SymbolKind::Struct => "struct",
        sky_sl::hir::symbol::SymbolKind::Value => "value",
        sky_sl::hir::symbol::SymbolKind::Layout => "layout",
    };

    Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: format!("## {}\r\n{}", name, kind),
        }),
        range: None,
    }
}

fn hover_primitive(primitive: &Primitive, workspace: &Workspace) -> Hover {
    let name = workspace.interned_name(primitive.name);

    let kind = match primitive.kind {
        sky_sl::hir::primitive::PrimitiveKind::Boolean => "bool",
        sky_sl::hir::primitive::PrimitiveKind::Integer => "int",
        sky_sl::hir::primitive::PrimitiveKind::FloatingPoint => "float",
    };

    Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: format!("## {}\r\n{}", name, kind),
        }),
        range: None,
    }
}
