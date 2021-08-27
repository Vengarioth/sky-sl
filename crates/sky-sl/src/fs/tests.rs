use camino::Utf8PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use super::db::*;

#[salsa::database(
    FileDatabaseStorage,
)]
#[derive(Default)]
pub struct TestDatabase {
    storage: salsa::Storage<TestDatabase>,
}

impl salsa::Database for TestDatabase {}

#[test]
fn it_works() {
    let mut db = TestDatabase::default();

    initialize(&mut db);
    let path = insert_file(&mut db, &Utf8PathBuf::from_str("foo/bar/baz.skysl").unwrap(), Arc::new("baz".to_owned()));
    let path = insert_file(&mut db, &Utf8PathBuf::from_str("foo/bar/foo.skysl").unwrap(), Arc::new("foo".to_owned()));

    let p = db.lookup_intern_path_data(path).parent().unwrap();

    dbg!(find_file_contents(&db, path));
    dbg!(find_file_contents(&db, p));
}
