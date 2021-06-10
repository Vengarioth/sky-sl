use crate::syn::cst::SyntaxKind;
use text_size::TextSize;

#[derive(Debug)]
pub struct Token {
    kind: SyntaxKind,
    len: TextSize,
}

impl Token {
    pub fn new(kind: SyntaxKind, len: TextSize) -> Self {
        Self {
            kind,
            len,
        }
    }

    pub fn kind(&self) -> SyntaxKind {
        self.kind
    }

    pub fn text_len(&self) -> TextSize {
        self.len
    }

    pub fn len(&self) -> usize {
        let len: u32 = self.len.into();
        len as usize
    }
}
