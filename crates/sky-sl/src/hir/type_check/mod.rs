use std::collections::HashMap;
use crate::hir::untyped;

mod env;
mod error;

pub use env::*;
pub use error::*;

#[derive(Debug)]
pub struct Scope {
    items: HashMap<String, ()>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn add(&mut self, kind: String) {
        self.items.insert(kind, ());
    }
}

fn discover_items(module: &untyped::Module, scope: &mut Scope) {
    for item in module.items.iter() {
        match item {
            untyped::ItemKind::Struct(struct_kind) => {
                scope.add(struct_kind.name.clone());
            },
            untyped::ItemKind::Function(function_kind) => {
                scope.add(function_kind.signature.name.clone());
            },
        }
    }
}

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
        let hir = db.hir(path);

        let mut scope = super::Scope::new();
        super::discover_items(&hir, &mut scope);

        let path = db.intern_ty_path_data(TyPathData::Root("Test".to_string()));
        let data = path.lookup(&db);
        dbg!(data);
    }
}
