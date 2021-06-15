use camino::Utf8Path;

#[derive(Debug)]
pub struct Document {
    path: Utf8Path,
    version: u32,
}

impl Document {
    pub fn new(path: Utf8Path, version: u32) -> Self {
        Self {
            path,
            version,
        }
    }
}
