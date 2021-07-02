use camino::{Utf8Path, Utf8PathBuf};
use std::collections::HashMap;

pub trait Package {
    fn resolve(&mut self, path: &Utf8Path);
}

#[derive(Debug)]
pub struct AdHocPackage {
    files: HashMap<Utf8PathBuf, String>,
}

impl AdHocPackage {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }

    pub fn add_file(&mut self, path: &Utf8Path, content: String) {
        self.files.insert(path.into(), content);
    }
}

impl Package for AdHocPackage {
    fn resolve(&mut self, _path: &Utf8Path) {
        
    }
}
