mod env;
mod error;

pub use env::*;
pub use error::*;

#[cfg(test)]
mod tests {
    use crate::db::*;
    use camino::Utf8PathBuf;
    use std::str::FromStr;
    use std::sync::Arc;

    #[test]
    fn it_works() {
        let mut db = CompilerDatabase::default();

        let path = Utf8PathBuf::from_str("/foo/bar").unwrap();
        let input = "struct Foo { bar: f32 } fn foo() { let a = Foo { bar: 0.0 }; }".to_string();
        db.set_input_file(path.clone(), Arc::from(input));

        let path = db.intern_ty_path_data(TyPathData::Root("Test".to_string()));
        let data = path.lookup(&db);
        dbg!(data);
    }
}
