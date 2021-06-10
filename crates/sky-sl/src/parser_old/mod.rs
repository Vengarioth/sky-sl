pub mod combinator;

mod cursor;
mod error;
mod lexer;
mod token_cursor;


pub use cursor::*;
pub use error::*;
pub use lexer::*;

use crate::ast::*;
use token_cursor::*;

pub fn parse(input: &str) -> Option<Vec<ItemKind>> {
    let token = tokenize(input).collect::<Vec<_>>();
    let mut cursor = TokenCursor::new(0, input, &token);
    let mut items = Vec::new();

    while !cursor.completed() {
        let (next, item) = parse_item(cursor)?;
        items.push(item);
        cursor = next;
    }

    Some(items)
}

fn calculate_advance(input: &str, mut line: u32, mut column: u32) -> (u32, u32) {
    for c in input.chars() {
        match c {
            '\u{000A}' => {
                line += 1;
                column = 1;
            },
            '\u{0009}' => (),
            '\u{000D}' => (),
            '\u{200E}' => (),
            '\u{200F}' => (),
            _ => {
                column += 1;
            },
        }
    }

    (line, column)
}

fn parse_item(cursor: TokenCursor) -> Option<(TokenCursor, ItemKind)> {
    if let Some((next, item)) = parse_struct(cursor) {
        return Some((next, ItemKind::Struct(item)));
    }

    if let Some((next, item)) = parse_function(cursor) {
        return Some((next, ItemKind::Function(item)));
    }

    None
}

fn parse_struct(cursor: TokenCursor) -> Option<(TokenCursor, StructKind)> {
    let cursor = skip_whitespace(cursor);
    let cursor = parse_keyword(cursor, "struct")?;

    let cursor = skip_whitespace(cursor);
    let (cursor, identifier) = parse_identifier(cursor)?;

    let cursor = skip_whitespace(cursor);
    let cursor = parse_token(cursor, TokenKind::OpenBrace)?;

    let cursor = skip_whitespace(cursor);
    let cursor = parse_token(cursor, TokenKind::CloseBrace)?;

    Some((cursor, StructKind {
        identifier,
    }))
}

fn parse_function(cursor: TokenCursor) -> Option<(TokenCursor, FunctionKind)> {
    let cursor = skip_whitespace(cursor);
    let cursor = parse_keyword(cursor, "fn")?;

    let cursor = skip_whitespace(cursor);
    let (cursor, identifier) = parse_identifier(cursor)?;
    
    let cursor = skip_whitespace(cursor);
    let cursor = parse_token(cursor, TokenKind::OpenParen)?;
    
    let cursor = skip_whitespace(cursor);
    let cursor = parse_token(cursor, TokenKind::CloseParen)?;

    let cursor = skip_whitespace(cursor);
    let cursor = parse_token(cursor, TokenKind::OpenBrace)?;

    let cursor = skip_whitespace(cursor);
    let cursor = parse_token(cursor, TokenKind::CloseBrace)?;

    Some((cursor, FunctionKind {
        identifier,
    }))
}

fn parse_identifier(cursor: TokenCursor) -> Option<(TokenCursor, Identifier)> {
    let (next, token) = cursor.next()?;

    if token.token.kind != TokenKind::Identifier {
        return None;
    }

    Some((next, Identifier { }))
}

fn parse_token<'a>(cursor: TokenCursor<'a>, expected: TokenKind) -> Option<TokenCursor<'a>> {
    let (next, token) = cursor.next()?;
    if token.token.kind != expected {
        return None;
    }

    Some(next)
}

fn parse_keyword<'c, 's>(cursor: TokenCursor<'c>, keyword: &'s str) -> Option<TokenCursor<'c>> {
    let (next, token) = cursor.next()?;
    if token.token.kind != TokenKind::Identifier {
        return None;
    }

    if token.input != keyword {
        return None;
    }

    Some(next)
}

fn skip_whitespace(mut cursor: TokenCursor) -> TokenCursor {
    while let Some((next, token)) = cursor.next() {
        if token.token.kind == TokenKind::Whitespace {
            cursor = next;
        } else {
            break;
        }
    }

    cursor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        dbg!(parse("struct Foo { } fn bar() {}"));
    }
}
