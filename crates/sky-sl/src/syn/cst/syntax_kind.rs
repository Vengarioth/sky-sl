#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[repr(u16)]
pub enum SyntaxKind {
    /// A module
    Module,

    /// A module declaration
    ModuleDeclaration,

    /// A use declaration
    UseDeclaration,

    UseTree,
    UseGroup,
    UseSegment,
    UseAll,

    Name,

    /// A layout
    Layout,

    /// A layout member list
    LayoutMemberList,

    /// A member of a LayoutMemberList
    LayoutMember,

    /// A binding index
    BindingIndex,

    /// A binding kind
    BindingKind,

    /// A struct
    Struct,

    /// A struct member list
    MemberList,

    /// A struct member
    Member,

    /// A function
    Fn,

    /// A function signature e.g. `fn example(argument: ArgumentType) -> ReturnType`
    FnSignature,

    /// A list of arguments
    ArgumentList,

    /// A single argument
    Argument,

    /// A return type declaration e.g. `-> SomeType`
    ReturnType,

    /// A path e.g. `foo::bar`
    Path,

    /// A path segment e.g. `foo`
    PathSegment,

    /// Block
    Block,

    /// A let binding statement e.g. `let a = 1 + 2;`
    LetStatement,

    /// A statement consisting of a single expression e.g. `my_fn();`
    ExpressionStatement,

    /// A grouping expression e.g. `(a + b)`
    GroupExpression,

    /// A path expression e.g. `a`, `a::b`
    PathExpression,

    /// A postfix field access expression e.g. `a.b`
    FieldAccessExpression,

    /// A literal expression
    LiteralExpression,

    /// A binary expression with two operands
    BinaryExpression,

    // A unary expression with only one operand
    UnaryExpression,

    /// A postfix call expression e.g. `a()` or `a.b()`
    CallExpression,

    /// A list of expressions used in a call e.g. `(1.0, 2.0)`
    CallArgumentList,

    /// A single call argument expression
    CallArgument,

    /// A postfix index expression e.g. `a[0]`
    IndexExpression,

    /// An expression that represents the value to index with, e.g. the `0` in `a[0]`
    Indexer,

    /// A struct expression e.g. `MyStruct { a: a, b: b }`
    StructExpression,

    /// A list of initializers for a struct expression e.g. `{ a: 1.0, b: 2.0 }`
    StructExpressionFields,

    /// A single struct expression field initializer e.g. `a: 10`
    StructExpressionField,

    /// An operator as part of an expression
    Operator,

    /// the "let" keyword
    LetKeyword,

    /// the "struct" keyword
    StructKeyword,

    /// the "fn" keyword
    FnKeyword,

    /// the "use" keyword
    UseKeyword,

    /// the "mod" keyword
    ModKeyword,

    /// the "package" keyword
    PackageKeyword,

    /// any non-keyword identifier
    Identifier,

    /// the "true" keyword
    TrueKeyword,

    /// the "false" keyword
    FalseKeyword,

    /// The "if" keyword
    IfKeyword,

    /// The "else" keyword
    ElseKeyword,

    /// The "loop" keyword
    LoopKeyword,

    /// The "while" keyword
    WhileKeyword,

    /// The "for" keyword
    ForKeyword,

    /// The "layout" keyword
    LayoutKeyword,

    /// the "binding" keyword
    BindingKeyword,

    /// the "uniform" keyword
    UniformKeyword,

    /// the "storage" keyword
    StorageKeyword,

    /// the "image" keyword
    ImageKeyword,

    /// the "sampler" keyword
    SamplerKeyword,

    /// any type-identifier (TODO remove when we have paths)
    TypeIdentifier,

    /// any whitespace
    Whitespace,

    /// any comment
    Comment,

    /// A boolean literal: `true` or `false`
    BoolLiteral,

    /// A numeric literal (TODO: split into different typed and untyped literals, e.g. 1f16, 0.0 or 42)
    NumLiteral,

    /// an integer literal, e.g. `42`
    IntLiteral,

    /// a floating point literal, e.g. `3.141592`
    FloatLiteral,

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
            StructKeyword | FnKeyword | UseKeyword | ModKeyword | LetKeyword | TrueKeyword
            | FalseKeyword | IfKeyword | ElseKeyword | LoopKeyword | WhileKeyword | ForKeyword
            | LayoutKeyword | BindingKeyword | UniformKeyword | StorageKeyword | ImageKeyword | SamplerKeyword => true,
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
            NumLiteral | FloatLiteral | IntLiteral | BoolLiteral => true,
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
            SyntaxKind::Equals => Some(Operator::Assign),

            SyntaxKind::Plus => Some(Operator::Add),
            SyntaxKind::Minus => Some(Operator::Subtract),
            SyntaxKind::Star => Some(Operator::Multiply),
            SyntaxKind::Slash => Some(Operator::Divide),

            SyntaxKind::Percent => Some(Operator::Remainder),

            SyntaxKind::And => Some(Operator::And),
            SyntaxKind::VerticalBar => Some(Operator::Or),
            SyntaxKind::Caret => Some(Operator::XOr),

            _ => None,
        }
    }

    pub fn from_keyword(ident: &str) -> Option<Self> {
        use self::SyntaxKind::*;

        match ident {
            "struct" => Some(StructKeyword),
            "fn" => Some(FnKeyword),
            "let" => Some(LetKeyword),
            "use" => Some(UseKeyword),
            "mod" => Some(ModKeyword),
            "package" => Some(PackageKeyword),
            "true" => Some(TrueKeyword),
            "false" => Some(FalseKeyword),
            "if" => Some(IfKeyword),
            "else" => Some(ElseKeyword),
            "loop" => Some(LoopKeyword),
            "while" => Some(WhileKeyword),
            "for" => Some(ForKeyword),
            "layout" => Some(LayoutKeyword),
            "binding" => Some(BindingKeyword),
            "uniform" => Some(UniformKeyword),
            "storage" => Some(StorageKeyword),
            "image" => Some(ImageKeyword),
            "sampler" => Some(SamplerKeyword),
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
    /// Assignment operator
    Assign,

    /// Arithmetic Add operator
    Add,

    /// Arithmetic Subtract operator
    Subtract,

    /// Arithmetic Multiply operator
    Multiply,

    /// Arithmetic Divide operator
    Divide,

    /// Remainder (modulo) operator
    Remainder,

    /// Logical AND operator
    And,

    /// Logical OR operator
    Or,

    /// Logical XOR operator
    XOr,
    // TODO left shift
    // TODO right shift
}

impl Operator {
    pub fn precedence(self) -> u8 {
        // TODO
        match self {
            Operator::Assign => 1,

            Operator::Add => 1,
            Operator::Subtract => 1,
            Operator::Multiply => 2,
            Operator::Divide => 2,

            Operator::Remainder => 1,

            Operator::And => 1,
            Operator::Or => 1,
            Operator::XOr => 1,
        }
    }

    pub fn associativity(self) -> Associativity {
        // TODO
        match self {
            Operator::Assign => Associativity::Left,

            Operator::Add => Associativity::Left,
            Operator::Subtract => Associativity::Left,
            Operator::Multiply => Associativity::Left,
            Operator::Divide => Associativity::Left,

            Operator::Remainder => Associativity::Left,

            Operator::And => Associativity::Left,
            Operator::Or => Associativity::Left,
            Operator::XOr => Associativity::Left,
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
