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

    initialize_fs(&mut db);
    let _path = insert_file(&mut db, &Utf8PathBuf::from_str("foo/bar/baz.skysl").unwrap(), Arc::new("aaa".to_owned()));
    let _path = insert_file(&mut db, &Utf8PathBuf::from_str("foo/baz.skysl").unwrap(), Arc::new("bbb".to_owned()));
    let _path = insert_file(&mut db, &Utf8PathBuf::from_str("baz.skysl").unwrap(), Arc::new("ccc".to_owned()));
    let _path = insert_file(&mut db, &Utf8PathBuf::from_str("foo/bar/foo.skysl").unwrap(), Arc::new("ddd".to_owned()));
    let path = insert_file(&mut db, &Utf8PathBuf::from_str("foo/bar/bar.skysl").unwrap(), Arc::new("ddd".to_owned()));

    remove_file(&mut db, &Utf8PathBuf::from_str("foo/bar/does-not-exist.skysl").unwrap());
    remove_file(&mut db, &Utf8PathBuf::from_str("foo/bar/foo.skysl").unwrap());

    let p = db.lookup_intern_path_data(path).parent().unwrap();

    dbg!(find_file_contents(&db, path));
    dbg!(find_file_contents(&db, p));

    println!("file system:");
    println!("");
    debug(&db);
    println!("");
    println!("consistency:");
    check_consistency(&db);
}
