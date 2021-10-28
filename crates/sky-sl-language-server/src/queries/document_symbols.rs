use sky_sl::{hir::symbol::{Symbol, SymbolList}, syn::cst::{LineIndex, TextRange}, workspace::Workspace};
use tower_lsp::lsp_types::*;

pub fn document_symbols2(workspace: &Workspace, symbols: SymbolList, line_index: LineIndex) -> DocumentSymbolResponse {
    DocumentSymbolResponse::Nested(
        symbols
            .iter()
            .map(|symbol| convert_symbol(workspace, symbol, &line_index))
            .collect(),
    )
}

fn convert_symbol(workspace: &Workspace, symbol: &Symbol, line_index: &LineIndex) -> DocumentSymbol {
    let range = span_to_range(symbol.span, &line_index);
    let selection_range = span_to_range(symbol.selection_span, &line_index);

    let kind = kind_to_lsp(symbol.kind);

    let children = if symbol.member.len() > 0 {
        let mut children = Vec::new();

        for member in &symbol.member {
            let range = span_to_range(member.span, &line_index);
            let selection_range = span_to_range(member.selection_span, &line_index);
            let name = workspace.interned_name(member.name);
            let kind = member_kind_to_lsp(member.kind);

            #[allow(deprecated)]
            children.push(DocumentSymbol {
                name,
                detail: None,
                kind,
                tags: None,
                range,
                selection_range,
                children: None,
                deprecated: None,
            })
        }
        
        Some(children)
    } else {
        None
    };

    let name = workspace.interned_name(symbol.name);

    #[allow(deprecated)]
    DocumentSymbol {
        name,
        detail: None,
        kind,
        tags: None,
        range,
        selection_range,
        children,
        deprecated: None,
    }
}

fn span_to_range(span: TextRange, line_index: &LineIndex) -> Range {
    let start = line_index.find_position(span.start());
    let end = line_index.find_position(span.end());
    Range::new(
        Position::new(start.line, start.column),
        Position::new(end.line, end.column),
    )
}

fn kind_to_lsp(kind: sky_sl::hir::symbol::SymbolKind) -> tower_lsp::lsp_types::SymbolKind {
    match kind {
        sky_sl::hir::symbol::SymbolKind::Module => tower_lsp::lsp_types::SymbolKind::Module,
        sky_sl::hir::symbol::SymbolKind::Function => tower_lsp::lsp_types::SymbolKind::Function,
        sky_sl::hir::symbol::SymbolKind::Struct => tower_lsp::lsp_types::SymbolKind::Struct,
        sky_sl::hir::symbol::SymbolKind::Value => tower_lsp::lsp_types::SymbolKind::Variable,
        sky_sl::hir::symbol::SymbolKind::Layout => tower_lsp::lsp_types::SymbolKind::Struct,
    }
}

fn member_kind_to_lsp(kind: sky_sl::hir::symbol::SymbolMemberKind) -> tower_lsp::lsp_types::SymbolKind {
    match kind {
        sky_sl::hir::symbol::SymbolMemberKind::Field => tower_lsp::lsp_types::SymbolKind::Field,
    }
}
