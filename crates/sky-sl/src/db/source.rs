use camino::Utf8PathBuf;
use std::sync::Arc;

#[salsa::query_group(SourceDatabaseStorage)]
pub trait SourceDatabase: salsa::Database {
    #[salsa::input]
    fn input_file(&self, name: Utf8PathBuf) -> Arc<str>;
}
