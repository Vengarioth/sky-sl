use rowan::{TextSize, TextRange};
use std::sync::Arc;

#[derive(Debug)]
pub struct LineTextRange {
    pub start: TextPosition,
    pub end: TextPosition,
}

#[derive(Debug)]
pub struct TextPosition {
    pub line: u32,
    pub column: u32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct LineIndex {
    newlines: Arc<Vec<TextSize>>,
}

impl LineIndex {
    pub fn from_str(input: &str) -> Self {
        let mut newlines = Vec::new();
        newlines.push(0.into());
        
        let mut row: TextSize = 0.into();

        for c in input.chars() {
            let char_length = TextSize::of(c);
            row += char_length;

            if c == '\n' {
                newlines.push(row);
                continue;
            }

            if !c.is_ascii() {
                println!("Warning, non ASCII character not yet supported: {}", c);
            }
        }

        Self {
            newlines: Arc::new(newlines),
        }
    }

    pub fn find_range(&self, range: TextRange) -> LineTextRange {
        LineTextRange {
            start: self.find_position(range.start()),
            end: self.find_position(range.end()),
        }
    }

    pub fn find_position(&self, offset: TextSize) -> TextPosition {
        let line = self.newlines.binary_search_by(|x| {
            if x <= &offset {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        })
        .unwrap_or_else(|i| i) - 1;

        let line_start_offset = self.newlines[line];
        let col = offset - line_start_offset;

        TextPosition {
            line: line as u32,
            column: col.into(),
        }
    }

    pub fn find_offset(&self, line: u32, character: u32) -> TextSize {
        let line = self.newlines[line as usize];
        let character: TextSize = character.into();
        TextSize::from(line + character)
    }
}
