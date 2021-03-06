use crate::syn::cst::SyntaxKind;
use rowan::TextSize;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ParseDiagnostic {
    SkippedToken {
        location: TextSize,
        skipped: SyntaxKind,
        expected: Vec<SyntaxKind>,
    },
    MissingToken {
        location: TextSize,
        expected: Vec<SyntaxKind>,
    },
}
