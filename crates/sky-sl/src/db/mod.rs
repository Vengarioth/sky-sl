mod source;
mod storage;
mod syntax;
mod hir;
mod ty;

pub use source::*;
pub use storage::*;
pub use syntax::*;
pub use hir::*;
pub use ty::*;

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
