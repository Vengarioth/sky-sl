use salsa::InternId;

pub mod lower;
pub mod type_check;
pub mod typed;
pub mod untyped;

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
        let input = "fn foo() { let a = 1 + 2 * 3; }".to_string();
        db.set_input_file(path.clone(), Arc::from(input));
        let hir = db.hir(path);

        dbg!(hir);
        // panic!();
    }
}
