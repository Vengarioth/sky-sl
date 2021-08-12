use super::ManifestDatabase;
use camino::Utf8PathBuf;
use std::sync::Arc;

use crate::syn::cst::LineIndex;

#[salsa::query_group(SourceDatabaseStorage)]
pub trait SourceDatabase: ManifestDatabase {
    #[salsa::input]
    fn input_file(&self, name: Utf8PathBuf) -> Arc<String>;

    fn line_index(&self, name: Utf8PathBuf) -> Arc<LineIndex>;
}

fn line_index(db: &dyn SourceDatabase, name: Utf8PathBuf) -> Arc<LineIndex> {
    let input = db.input_file(name);
    Arc::new(LineIndex::new(&input))
}
