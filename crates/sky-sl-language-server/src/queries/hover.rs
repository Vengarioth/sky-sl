use sky_sl::{hir::symbol::SymbolList, syn::cst::LineIndex};
use tower_lsp::lsp_types::*;

pub fn hover(_symbols: SymbolList, _position: Position, _line_index: LineIndex) -> Hover {
    Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: "## Hello World\r\nhello.".to_string(),
        }),
        range: None,
    }
}
