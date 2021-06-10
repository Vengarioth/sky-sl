use std::num::NonZeroU32;

mod green;
mod syntax_kind;
mod trivia;

pub use green::*;
pub use syntax_kind::*;
pub use trivia::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct SyntaxId(NonZeroU32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Lang {}
impl rowan::Language for Lang {
    type Kind = SyntaxKind;
    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= SyntaxKind::Error as u16);
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }
    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

type SyntaxNode = rowan::SyntaxNode<Lang>;
type SyntaxToken = rowan::SyntaxToken<Lang>;
type SyntaxElement = rowan::NodeOrToken<SyntaxNode, SyntaxToken>;
type Builder<'a> = rowan::GreenNodeBuilder<'a>;
