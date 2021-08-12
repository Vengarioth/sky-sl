use camino::Utf8PathBuf;
use std::sync::Arc;
use salsa::{InternId, InternKey};

#[salsa::query_group(PackageDatabaseStorage)]
pub trait PackageDatabase: salsa::Database {
    #[salsa::interned]
    fn intern_package(&self, data: Arc<PackageData>) -> Package;

    fn root_path(&self, package: Package) -> Utf8PathBuf;
}

fn root_path(db: &dyn PackageDatabase, package: Package) -> Utf8PathBuf {
    let package_data = db.lookup_intern_package(package);
    package_data.root.clone()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Package(InternId);

impl InternKey for Package {
    fn from_intern_id(v: InternId) -> Self {
        Self(v)
    }

    fn as_intern_id(&self) -> InternId {
        self.0
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct PackageData {
    pub root: Utf8PathBuf,
}
