use super::{Token, TokenKind};

#[derive(Debug, Copy, Clone)]
pub struct ParseToken<'a> {
    offset: usize,
    pub token: &'a Token,
    pub input: &'a str,
}

#[derive(Debug, Copy, Clone)]
pub struct TokenCursor<'a> {
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

    /// returns if at most whitespace remains
    pub fn completed(&self) -> bool {
        self.token.len() == 0 || self.token.iter().all(|t| t.kind == TokenKind::Whitespace)
    }

    pub fn peek(&self) -> Option<ParseToken> {
        if self.token.len() == 0 {
            return None;
        }

        let token = &self.token[0];
        let input = &self.input[0..token.length];
        let offset = self.offset;

        Some(ParseToken { offset, token, input })
    }

    pub fn next(&self) -> Option<(Self, ParseToken)> {
        if self.token.len() == 0 {
            return None;
        }

        let token = &self.token[0];
        let input = &self.input[0..token.length];
        let offset = self.offset;
        let remaining_token = &self.token[1..];
        let remaining_input = &self.input[token.length..];
        let next_offset = offset + token.length;

        let next = Self::new(next_offset, remaining_input, remaining_token);
        let kind = ParseToken { offset, token, input };

        Some((next, kind))
    }
}
