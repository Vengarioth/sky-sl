use std::num::NonZeroU32;

mod line_index;
mod syntax_kind;
mod trivia;

pub use line_index::*;
pub use syntax_kind::*;
pub use trivia::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct SyntaxId(NonZeroU32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Lang {}
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

pub type GreenNode = rowan::GreenNode;
pub type SyntaxNode = rowan::SyntaxNode<Lang>;
pub type SyntaxNodeChildren = rowan::SyntaxNodeChildren<Lang>;
pub type SyntaxToken = rowan::SyntaxToken<Lang>;
pub type SyntaxElement = rowan::NodeOrToken<SyntaxNode, SyntaxToken>;
pub type Builder<'a> = rowan::GreenNodeBuilder<'a>;
pub type Checkpoint = rowan::Checkpoint;
pub use rowan::{WalkEvent};
