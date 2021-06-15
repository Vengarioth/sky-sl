mod source;
mod storage;
mod syntax;

pub use source::*;
pub use storage::*;
pub use syntax::*;

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use camino::*;
    use super::*;
    use std::sync::Arc;
    use crate::syn::ast::*;

    #[test]
    fn it_works() {
        let mut db = CompilerDatabase::default();
        
        let path = Utf8PathBuf::from_str("/foo/bar").unwrap();
        let input = "fn bar(x: y) {} fn foo() {}".to_string();
        db.set_input_file(path.clone(), Arc::from(input));
        db.ast(path.clone());

        let ast: Arc<Root> = db.ast(path.clone());
        
        for item in ast.module_items() {
            dbg!(item);
        }

        for fndef in ast.function_definitions() {
            dbg!(fndef.identifier());
            
            if let Some(argument_list) = fndef.argument_list() {
                for argument in argument_list.arguments() {
                    dbg!(argument);
                }
            }

            dbg!(fndef);
        }

        for structdef in ast.struct_definitions() {
            dbg!(structdef.identifier());
            dbg!(structdef);
        }

        panic!();
    }
}
