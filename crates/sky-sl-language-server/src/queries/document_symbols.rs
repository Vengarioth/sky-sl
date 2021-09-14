use sky_sl::{
    hir::symbol::{Symbol, SymbolList},
    syn::cst::{LineIndex, TextRange},
};
use tower_lsp::lsp_types::*;

pub fn document_symbols2(symbols: SymbolList, line_index: LineIndex) -> DocumentSymbolResponse {
    DocumentSymbolResponse::Nested(
        symbols
            .iter()
            .map(|symbol| convert_symbol(symbol, &line_index))
            .collect(),
    )
}

fn convert_symbol(symbol: &Symbol, line_index: &LineIndex) -> DocumentSymbol {
    let range = span_to_range(symbol.span, &line_index);
    let selection_range = span_to_range(symbol.selection_span, &line_index);

    let kind = kind_to_lsp(symbol.kind);

    let children = if let Some(children) = &symbol.children {
        Some(
            children
                .iter()
                .map(|s| convert_symbol(s, line_index))
                .collect(),
        )
    } else {
        None
    };

    #[allow(deprecated)]
    DocumentSymbol {
        name: symbol.name.to_owned(),
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
        sky_sl::hir::symbol::SymbolKind::Field => tower_lsp::lsp_types::SymbolKind::Field,
        sky_sl::hir::symbol::SymbolKind::Value => tower_lsp::lsp_types::SymbolKind::Variable,
    }
}
