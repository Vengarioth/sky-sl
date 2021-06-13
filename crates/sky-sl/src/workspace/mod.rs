use camino::Utf8PathBuf;

use crate::syn::cst::LineIndex;

#[derive(Debug)]
pub struct Workspace {
    root: Utf8PathBuf,
}

impl Workspace {
    pub fn new(root: Utf8PathBuf) -> Self {
        Self {
            root,
        }
    }

    pub fn document_symbols(&self, path: Utf8PathBuf) -> Result<crate::parser::ParseResult, ()> {
        let input = std::fs::read_to_string(path).expect("could not read file to string");
        let token = crate::lexer::tokenize(&input);
        Ok(crate::parser::parse(&token, &input))
    }

    pub fn get_line_index(&self, path: Utf8PathBuf) -> Result<LineIndex, ()> {
        let input = std::fs::read_to_string(path).expect("could not read file to string");
        Ok(LineIndex::new(&input))
    }
}
