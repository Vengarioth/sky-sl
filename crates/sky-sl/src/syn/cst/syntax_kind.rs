#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[repr(u16)]
pub enum SyntaxKind {

    /// A module
    Module,

    /// A struct
    Struct,

    /// A struct member list
    MemberList,

    /// A struct member
    Member,

    /// A function
    Fn,

    /// A list of arguments
    ArgumentList,

    /// A single argument
    Argument,

    /// Block
    Block,

    /// A let binding statement e.g. `let a = 1 + 2;`
    LetStatement,

    /// A grouping expression e.g. `(a + b)`
    GroupExpression,

    /// A path expression e.g. `a`, `a::b`
    PathExpression,

    /// A literal expression
    LiteralExpression,

    /// A binary expression with two operands
    BinaryExpression,

    /// An operator as part of an expression
    Operator,

    /// the "let" keyword
    LetKeyword,

    /// the "struct" keyword
    StructKeyword,

    /// the "fn" keyword
    FnKeyword,

    /// any non-keyword identifier
    Identifier,

    /// any type-identifier (TODO remove when we have paths)
    TypeIdentifier,

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

    /// End of file
    EOF,

    /// Error
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

    pub fn operator(self) -> Option<Operator> {
        match self {
            SyntaxKind::Plus => Some(Operator::Add),
            SyntaxKind::Minus => Some(Operator::Subtract),
            SyntaxKind::Star => Some(Operator::Multiply),
            SyntaxKind::Slash => Some(Operator::Divide),
            _ => None,
        }
    }

    pub fn from_keyword(ident: &str) -> Option<Self> {
        use self::SyntaxKind::*;

        match ident {
            "struct" => Some(StructKeyword),
            "fn" => Some(FnKeyword),
            "let" => Some(LetKeyword),
            _ => None,
        }
    }
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operator {
    pub fn precedence(self) -> u8 {
        match self {
            Operator::Add => 0,
            Operator::Subtract => 0,
            Operator::Multiply => 0,
            Operator::Divide => 0,
        }
    }

    pub fn associativity(self) -> Associativity {
        match self {
            Operator::Add => Associativity::Left,
            Operator::Subtract => Associativity::Left,
            Operator::Multiply => Associativity::Left,
            Operator::Divide => Associativity::Left,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Associativity {
    Left,
    Right,
}

impl Associativity {
    pub fn left(self) -> bool {
        match self {
            Associativity::Left => true,
            Associativity::Right => false,
        }
    }

    pub fn right(self) -> bool {
        !self.left()
    }
}
