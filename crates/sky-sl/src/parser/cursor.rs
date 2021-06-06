use std::str::Chars;

#[derive(Debug)]
pub struct Cursor<'a> {
    initial_len: usize,
    chars: Chars<'a>,
}

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            initial_len: input.len(),
            chars: input.chars(),
        }
    }

    fn chars(&self) -> Chars<'a> {
        self.chars.clone()
    }

    fn nth_char(&self, n: usize) -> Option<char> {
        self.chars().nth(n)
    }

    pub fn len_consumed(&self) -> usize {
        self.initial_len - self.chars.as_str().len()
    }

    pub fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while let Some(c) = self.first() {
            if !predicate(c) {
                break;
            }

            self.bump();
        }
    }

    pub fn first(&self) -> Option<char> {
        self.nth_char(0)
    }

    pub fn bump(&mut self) -> Option<char> {
        self.chars.next()
    }
}
