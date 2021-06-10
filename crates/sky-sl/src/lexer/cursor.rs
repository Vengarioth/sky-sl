use std::str::Chars;

use text_size::TextSize;

#[derive(Debug)]
pub struct Cursor<'a> {
    input: &'a str,
    len: TextSize,
}

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            len: 0.into(),
        }
    }

    pub fn len(&self) -> TextSize {
        self.len
    }

    pub fn first(&self) -> Option<char> {
        self.chars().next()
    }

    pub fn chars(&self) -> Chars {
        let len: u32 = self.len.into();
        self.input[len as usize..].chars()
    }

    pub fn bump(&mut self) -> Option<char> {
        let c = self.chars().next()?;
        self.len += TextSize::of(c);
        Some(c)
    }

    pub fn current_text(&self) -> &str {
        let len: u32 = self.len.into();
        &self.input[..len as usize]
    }

    pub fn bump_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while let Some(c) = self.first() {
            if !predicate(c) {
                break;
            }

            self.bump();
        }
    }
}
