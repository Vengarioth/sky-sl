mod source;
mod storage;
mod syntax;
mod hir;
mod ty;
mod module;
mod package;
mod manifest;

pub use source::*;
pub use storage::*;
pub use syntax::*;
pub use hir::*;
pub use ty::*;
pub use module::*;
pub use package::*;
pub use manifest::*;

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
