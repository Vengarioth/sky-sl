#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Span {
    pub offset: u32,
    pub length: u32,
}

impl Span {
    pub fn new(offset: u32, length: u32) -> Self {
        Self {
            offset,
            length,
        }
    }
}
