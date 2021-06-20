use sky_sl::syn::cst::LineTextRange;
use tower_lsp::lsp_types::SemanticToken;

#[derive(Debug)]
pub struct DeltaEncoder {
    line: u32,
}

impl DeltaEncoder {
    pub fn new() -> Self {
        Self {
            line: 0,
        }
    }

    pub fn create_next(&mut self, range: LineTextRange, token_type: u32, token_modifiers_bitset: u32) -> SemanticToken {
        let delta_line = range.start.line - self.line;
        let delta_start = range.start.column;

        debug_assert!(range.start.line == range.end.line);
        let length = range.end.column - range.start.column;

        self.line = range.start.line;

        SemanticToken {
            delta_line,
            delta_start,
            length,
            token_type,
            token_modifiers_bitset,
        }
    }
}
