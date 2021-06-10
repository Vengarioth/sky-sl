/// Stores whitespace information around a Token.
/// Tokens own the trivia after it up to the first new line
/// Tokens own the trivia before it starting with the first new line
#[derive(Debug)]
pub struct Trivia {
    pub spaces: u32,
    pub newlines: u32
}

impl Trivia {
    pub fn new(spaces: u32, newlines: u32) -> Self {
        Self {
            spaces,
            newlines,
        }
    }
}

impl std::fmt::Display for Trivia {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for _ in 0..self.newlines {
            write!(f, "\r\n")?;
        }

        for _ in 0..self.spaces {
            write!(f, " ")?;
        }

        Ok(())
    }
}
