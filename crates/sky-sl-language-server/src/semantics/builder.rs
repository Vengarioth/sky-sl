use sky_sl::syn::cst::*;
use tower_lsp::lsp_types::*;

pub struct SemanticTokensBuilder<'a> {
    line_index: &'a LineIndex,
    line: u32,
    tokens: Vec<SemanticToken>,
}

impl<'a> SemanticTokensBuilder<'a> {
    pub fn new(line_index: &'a LineIndex) -> Self {
        Self {
            line_index,
            line: 0,
            tokens: Vec::new(),
        }
    }

    pub fn build_token(&mut self, range: TextRange, token_type: u32, token_modifiers_bitset: u32) {
        let range = self.line_index.find_range(range);
        let delta_line = range.start.line - self.line;
        let delta_start = range.start.column;

        debug_assert!(range.start.line == range.end.line);
        let length = range.end.column - range.start.column;

        self.line = range.start.line;

        self.tokens.push(SemanticToken {
            delta_line,
            delta_start,
            length,
            token_type,
            token_modifiers_bitset,
        });
    }

    pub fn finish(self) -> Vec<SemanticToken> {
        self.tokens
    }
}
