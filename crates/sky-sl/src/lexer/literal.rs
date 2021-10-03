use crate::syn::cst::SyntaxKind;
use super::Cursor;

pub fn is_num_literal_start(c: char) -> bool {
    ('0'..='9').contains(&c)
}

impl Cursor<'_> {
    pub(super) fn num_literal(&mut self) -> SyntaxKind {
        self.bump_while(is_num_literal_start);

        if let Some('.') = self.first() {
            self.bump();
            self.bump_while(is_num_literal_start);
            SyntaxKind::FloatLiteral
        } else {
            SyntaxKind::IntLiteral
        }
    }
}
