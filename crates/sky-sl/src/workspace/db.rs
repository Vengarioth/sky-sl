use crate::fs::db::*;
use crate::package::*;
use crate::syn::db::*;
use std::fmt;

#[salsa::database(
    FileDatabaseStorage,
    PackageDatabaseStorage,
    SyntaxDatabaseStorage,
)]
#[derive(Default)]
pub struct CompilerDatabase {
    storage: salsa::Storage<CompilerDatabase>,
}

impl salsa::Database for CompilerDatabase {}

impl fmt::Debug for CompilerDatabase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CompilerDatabase")
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_is_send() {
        let db = CompilerDatabase::default();
        std::thread::spawn(move || {
            // move db into closure to test if everything is send
            let _ = db;
        });
    }
}
