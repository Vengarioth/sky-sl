use camino::{Utf8Path, Utf8PathBuf};

use crate::syn::cst::LineIndex;

#[derive(Debug)]
pub struct Workspace {
    root: Utf8PathBuf,
}

impl Workspace {
    /// Looks for a SkySL workspace in the file system by recursively looking for a workspace defining file upwards in the file system
    pub fn new_from_file_system_hierachy(query: Utf8PathBuf) -> Option<Self> {
        let workspace_path = Self::find_workspace_file(&query)?;
        Some(Self::new(workspace_path))
    }

    fn find_workspace_file(query: &Utf8Path) -> Option<Utf8PathBuf> {
        if query.is_dir() {
            if let Some(result) = Self::find_workspace_file(&query.join("skysl.workspace")) {
                Some(result)
            } else if let Some(parent) = query.parent() {
                Self::find_workspace_file(parent)
            } else {
                None
            }
        } else {
            if query.ends_with("skysl.workspace") {
                return Some(query.into());
            } else if let Some(parent) = query.parent() {
                Self::find_workspace_file(parent)
            } else {
                None
            }
        }
    }

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
