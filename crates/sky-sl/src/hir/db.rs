#[salsa::query_group(HirDatabaseStorage)]
pub trait HirDatabase {
    fn hir(&self, name: Utf8PathBuf) -> untyped::Module;
}
