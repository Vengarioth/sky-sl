mod cursor;
mod error;
mod lexer;

pub use cursor::*;
pub use error::*;
pub use lexer::*;

use crate::ast::*;

pub fn parse(input: &str) {
    let token = tokenize(input).collect::<Vec<_>>();
    let mut cursor = TokenCursor::new(0, input, &token);

    while let Some((item, next)) = parse_item(cursor) {
        dbg!(item);
        cursor = next;
    }
}

#[derive(Debug, Copy, Clone)]
struct TokenCursor<'a> {
    offset: usize,
    input: &'a str,
    token: &'a [Token],
}

impl<'a> TokenCursor<'a> {
    pub fn new(offset: usize, input: &'a str, token: &'a [Token]) -> Self {
        Self {
            offset,
            input,
            token,
        }
    }

    fn current(&self) -> &'a Token {
        &self.token[0]
    }

    fn current_str(&self) -> &'a str {
        &self.input[0..self.token[0].length]
    }

    fn next(&self) -> Option<Self> {
        if self.token.len() > 1 {
            let current = &self.token[0];
            let offset = self.offset + current.length;
            Some(Self::new(offset, &self.input[current.length..], &self.token[1..]))
        } else {
            None
        }
    }
}


fn parse_item(cursor: TokenCursor) -> Option<(ItemKind, TokenCursor)> {
    if let Some((item, cursor)) = parse_struct(cursor) {
        return Some((ItemKind::Struct(item), cursor));
    }

    if let Some((item, cursor)) = parse_function(cursor) {
        return Some((ItemKind::Function(item), cursor));
    }

    None
}

fn parse_struct(cursor: TokenCursor) -> Option<(StructKind, TokenCursor)> {
    let cursor = skip_whitespace(cursor)?;
    let cursor = parse_keyword(cursor, "struct")?;
    let cursor = skip_whitespace(cursor)?;
    let (identifier, cursor) = parse_identifier(cursor)?;
    let cursor = skip_whitespace(cursor)?;

    Some((StructKind {
        identifier,
    }, cursor))
}

fn parse_function(cursor: TokenCursor) -> Option<(FunctionKind, TokenCursor)> {
    let cursor = skip_whitespace(cursor)?;
    let cursor = parse_keyword(cursor, "fn")?;
    let cursor = skip_whitespace(cursor)?;
    let (identifier, cursor) = parse_identifier(cursor)?;
    let cursor = skip_whitespace(cursor)?;

    Some((FunctionKind {
        identifier,
    }, cursor))
}

fn parse_identifier(cursor: TokenCursor) -> Option<(Identifier, TokenCursor)> {
    if cursor.current().kind != TokenKind::Identifier {
        return None;
    }

    Some((Identifier {

    }, cursor.next()?))
}

fn parse_keyword<'c, 's>(cursor: TokenCursor<'c>, keyword: &'s str) -> Option<TokenCursor<'c>> {
    if cursor.current().kind != TokenKind::Identifier {
        return None;
    }

    if cursor.current_str() != keyword {
        return None;
    }

    return cursor.next()
}

fn skip_whitespace(mut cursor: TokenCursor) -> Option<TokenCursor> {
    while cursor.current().kind == TokenKind::Whitespace {
        if let Some(next) = cursor.next() {
            cursor = next;
        } else {
            return Some(cursor);
        }
    }

    Some(cursor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        parse("struct Foo { bar: f32 }");
    }
}
