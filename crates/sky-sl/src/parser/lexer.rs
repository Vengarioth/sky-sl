use super::cursor::Cursor;

#[derive(Debug, Eq, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub length: usize,
}

impl Token {
    pub fn new(kind: TokenKind, length: usize) -> Self {
        Self {
            kind,
            length,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum TokenKind {
    /// "// comment"
    LineComment,

    /// "/* block comment */"
    BlockComment,

    /// Any sequence of whitespace characters
    Whitespace,
    
    /// An identifier, including keywords
    Identifier,
    
    /// A numeric literal
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

    /// Character sequence not recognized by the lexer
    Unknown,
}

/// From https://github.com/rust-lang/rust/blob/master/compiler/rustc_lexer/src/lib.rs#L235
fn is_whitespace(c: char) -> bool {
    match c {
        '\u{0009}' => true,
        '\u{000A}' => true,
        '\u{000B}' => true,
        '\u{000C}' => true,
        '\u{000D}' => true,
        '\u{0020}' => true,
        '\u{0085}' => true,
        '\u{200E}' => true,
        '\u{200F}' => true,
        '\u{2028}' => true,
        '\u{2029}' => true,
        _ => false,
    }
}

/// from https://github.com/rust-lang/rust/blob/master/compiler/rustc_lexer/src/lib.rs#L267
fn is_identifier_start(c: char) -> bool {
    ('a'..='z').contains(&c)
    || ('A'..='Z').contains(&c)
    || c == '_'
    || (c > '\x7f' && unicode_xid::UnicodeXID::is_xid_start(c))
}

/// https://github.com/rust-lang/rust/blob/master/compiler/rustc_lexer/src/lib.rs#L279
fn is_identifier_continue(c: char) -> bool {
    ('a'..='z').contains(&c)
    || ('A'..='Z').contains(&c)
    || ('0'..='9').contains(&c)
    || c == '_'
    || (c > '\x7f' && unicode_xid::UnicodeXID::is_xid_start(c))
}

fn is_num_literal_start(c: char) -> bool {
    ('0'..='9').contains(&c)
}

fn is_num_literal_continue(c: char) -> bool {
    ('0'..='9').contains(&c)
    || c == '.'
}

pub fn tokenize(mut input: &str) -> impl Iterator<Item = Token> + '_ {
    std::iter::from_fn(move || {
        if input.is_empty() {
            return None;
        }

        let token = first_token(input);
        input = &input[token.length..];
        Some(token)
    })
}

fn first_token(input: &str) -> Token {
    debug_assert!(!input.is_empty());
    Cursor::new(input).advance_token()
}

impl Cursor<'_> {
    fn advance_token(&mut self) -> Token {
        let first_char = self.bump().unwrap();
        let token_kind = match first_char {
            '/' => match self.first() {
                Some('/') => self.line_comment(),
                Some('*') => self.block_comment(),
                _ => TokenKind::Slash,
            },
            c if is_whitespace(c) => self.whitespace(),
            c if is_identifier_start(c) => self.identifier(),
            
            // TODO number literals
            c if is_num_literal_start(c) => self.num_literal(),

            ';' => TokenKind::Semicolon,
            ',' => TokenKind::Comma,
            '.' => TokenKind::Dot,
            '(' => TokenKind::OpenParen,
            ')' => TokenKind::CloseParen,
            '{' => TokenKind::OpenBrace,
            '}' => TokenKind::CloseBrace,
            '[' => TokenKind::OpenBracket,
            ']' => TokenKind::CloseBracket,
            '@' => TokenKind::At,
            '#' => TokenKind::Pound,
            '~' => TokenKind::Tilde,
            '?' => TokenKind::Question,
            ':' => TokenKind::Colon,
            '$' => TokenKind::Dollar,
            '=' => TokenKind::Equals,
            '!' => TokenKind::Bang,
            '<' => TokenKind::LessThan,
            '>' => TokenKind::GreatherThan,
            '-' => TokenKind::Minus,
            '&' => TokenKind::And,
            '|' => TokenKind::VerticalBar,
            '+' => TokenKind::Plus,
            '*' => TokenKind::Star,
            '^' => TokenKind::Caret,
            '%' => TokenKind::Percent,

            _ => TokenKind::Unknown,
        };


        Token::new(token_kind, self.len_consumed())
    }

    fn line_comment(&mut self) -> TokenKind {
        // consume "/"
        self.bump();
        self.eat_while(|c| c != '\n');

        TokenKind::LineComment
    }

    fn block_comment(&mut self) -> TokenKind {
        // consume "*"
        self.bump();

        while let Some(c) = self.bump() {
            match c {
                '*' if self.first() == Some('/') => {
                    // consume "/"
                    self.bump();
                    break;
                },
                _ => (),
            }
        }

        TokenKind::BlockComment
    }

    fn whitespace(&mut self) -> TokenKind {
        self.eat_while(is_whitespace);
        TokenKind::Whitespace
    }

    fn identifier(&mut self) -> TokenKind {
        self.eat_while(is_identifier_continue);
        TokenKind::Identifier
    }

    fn num_literal(&mut self) -> TokenKind {
        self.eat_while(is_num_literal_continue);
        TokenKind::NumLiteral
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let _ = tokenize("/* */ hello world struct Foo { } fn bar() -> Baz { return null; }").collect::<Vec<_>>();

        // Line comment
        assert_eq!(
            tokenize("// Test").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::LineComment, 7)],
        );

        // Block comment
        assert_eq!(
            tokenize("/* Test */").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::BlockComment, 10)],
        );

        // Whitespace
        assert_eq!(
            tokenize("\r\n").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::Whitespace, 2)],
        );

        // Identifier
        assert_eq!(
            tokenize("Foo").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::Identifier, 3)],
        );

        // Numeric literals
        assert_eq!(
            tokenize("0").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::NumLiteral, 1)],
        );

        assert_eq!(
            tokenize("1234567890.0123456789").collect::<Vec<_>>(),
            vec![Token::new(TokenKind::NumLiteral, 21)],
        );
    }

    #[test]
    fn it_does_not_panic_when_block_comment_has_no_end() {
        let _ = tokenize("/*").collect::<Vec<_>>();
    }
}
