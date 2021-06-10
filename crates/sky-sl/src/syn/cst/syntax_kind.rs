#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[repr(u16)]
pub enum SyntaxKind {

    /// A module
    Module,

    /// A struct
    Struct,

    /// A function
    Fn,

    /// the "struct" keyword
    StructKeyword,

    /// the "fn" keyword
    FnKeyword,

    /// any non-keyword identifier
    Identifier,

    /// any whitespace
    Whitespace,

    /// any comment
    Comment,

    /// A numeric literal (TODO: split into different typed and untyped literals, e.g. 1f16, 0.0 or 42)
    NumLiteral,

    /// ";"
    Semicolon,

    /// ","
    Comma,

    /// "."
    Dot,

    /// "("
    OpenParen,

    /// ")"
    CloseParen,

    /// "{"
    OpenBrace,

    /// "}"
    CloseBrace,

    /// "["
    OpenBracket,

    /// "]"
    CloseBracket,

    /// "@"
    At,

    /// "#"
    Pound,

    /// "~"
    Tilde,

    /// "?"
    Question,

    /// ":"
    Colon,

    /// "$"
    Dollar,

    /// "="
    Equals,

    /// "!"
    Bang,

    /// "<"
    LessThan,

    /// ">"
    GreatherThan,

    /// "-"
    Minus,

    /// "&"
    And,

    /// "|"
    VerticalBar,

    /// "+"
    Plus,

    /// "*"
    Star,

    /// "/"
    Slash,

    /// "^"
    Caret,

    /// "%"
    Percent,

    Error,
}

impl SyntaxKind {
    pub fn is_keyword(self) -> bool {
        use self::SyntaxKind::*;

        match self {
            StructKeyword | FnKeyword => true,
            _ => false,
        }
    }

    pub fn is_puctuation(self) -> bool {
        use self::SyntaxKind::*;

        match self {
            Semicolon | Comma | Dot | OpenParen | CloseParen | OpenBrace | CloseBrace
            | OpenBracket | CloseBracket | At | Pound | Tilde | Question | Colon | Dollar
            | Equals | Bang | LessThan | GreatherThan | Minus | And | VerticalBar | Plus | Star
            | Slash | Caret | Percent => true,
            _ => false,
        }
    }

    pub fn is_literal(self) -> bool {
        use self::SyntaxKind::*;

        match self {
            NumLiteral => true,
            _ => false,
        }
    }

    pub fn is_whitespace(self) -> bool {
        use self::SyntaxKind::*;

        match self {
            Whitespace => true,
            _ => false,
        }
    }

    pub fn is_comment(self) -> bool {
        use self::SyntaxKind::*;

        match self {
            Comment => true,
            _ => false,
        }
    }

    pub fn from_keyword(ident: &str) -> Option<Self> {
        use self::SyntaxKind::*;

        match ident {
            "struct" => Some(StructKeyword),
            "fn" => Some(FnKeyword),
            _ => None,
        }
    }
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}
