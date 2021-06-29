use crate::syn::cst::SyntaxKind;

mod cursor;
mod identifier;
mod literal;
mod token;

use cursor::*;
use identifier::*;
use literal::*;
pub use token::*;


pub fn tokenize(mut input: &str) -> Vec<Token> {
    let mut result = Vec::new();
    while !input.is_empty() {
        let token = first_token(input);
        input = &input[token.len()..];
        result.push(token);
    }
    result
}

fn first_token(input: &str) -> Token {
    debug_assert!(!input.is_empty());
    Cursor::new(input).advance_token()
}

impl Cursor<'_> {
    fn advance_token(&mut self) -> Token {
        let first_char = self.bump().unwrap();
        let syntax_kind = match first_char {
            '/' => match self.first() {
                Some('/') => self.line_comment(),
                Some('*') => self.block_comment(),
                _ => SyntaxKind::Slash,
            },
            c if c.is_whitespace() => self.whitespace(),
            c if is_identifier_start(c) => self.identifier(),
            
            // TODO number literals
            c if is_num_literal_start(c) => self.num_literal(),

            ';' => SyntaxKind::Semicolon,
            ',' => SyntaxKind::Comma,
            '.' => SyntaxKind::Dot,
            '(' => SyntaxKind::OpenParen,
            ')' => SyntaxKind::CloseParen,
            '{' => SyntaxKind::OpenBrace,
            '}' => SyntaxKind::CloseBrace,
            '[' => SyntaxKind::OpenBracket,
            ']' => SyntaxKind::CloseBracket,
            '@' => SyntaxKind::At,
            '#' => SyntaxKind::Pound,
            '~' => SyntaxKind::Tilde,
            '?' => SyntaxKind::Question,
            ':' => SyntaxKind::Colon,
            '$' => SyntaxKind::Dollar,
            '=' => SyntaxKind::Equals,
            '!' => SyntaxKind::Bang,
            '<' => SyntaxKind::LessThan,
            '>' => SyntaxKind::GreatherThan,
            '-' => SyntaxKind::Minus,
            '&' => SyntaxKind::And,
            '|' => SyntaxKind::VerticalBar,
            '+' => SyntaxKind::Plus,
            '*' => SyntaxKind::Star,
            '^' => SyntaxKind::Caret,
            '%' => SyntaxKind::Percent,

            _ => SyntaxKind::Error,
        };


        Token::new(syntax_kind, self.len())
    }

    fn line_comment(&mut self) -> SyntaxKind {
        // consume "/"
        self.bump();
        self.bump_while(|c| c != '\n');

        SyntaxKind::Comment
    }

    fn block_comment(&mut self) -> SyntaxKind {
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

        SyntaxKind::Comment
    }

    fn whitespace(&mut self) -> SyntaxKind {
        self.bump_while(|c| c.is_whitespace());
        SyntaxKind::Whitespace
    }

    fn identifier(&mut self) -> SyntaxKind {
        self.bump_while(is_identifier_continue);
        SyntaxKind::from_keyword(self.current_text()).unwrap_or(SyntaxKind::Identifier)
    }

    fn num_literal(&mut self) -> SyntaxKind {
        self.bump_while(is_num_literal_continue);
        SyntaxKind::NumLiteral
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    fn it_works() {
        tokenize("/* */ hello world struct Foo { } fn bar() -> Baz { return null; }");
    }
}
