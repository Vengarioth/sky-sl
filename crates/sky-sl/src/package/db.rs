use super::{Package, Manifest};
use crate::fs::{FileDatabase, FileId, PathSegment};
use std::str::FromStr;
use std::sync::Arc;

const PACKAGE_MANIFEST_NAME: &'static str = "skysl.toml";

#[salsa::query_group(PackageDatabaseStorage)]
pub trait PackageDatabase: FileDatabase {
    /// Returns all packages found in the current workspace
    fn find_packages(&self) -> Vec<Package>;

    /// Returns the package parsed from the given file id
    fn package(&self, file: FileId) -> Package;

    /// Returns the file id corresponding to the source root
    fn source_root(&self, file: FileId) -> Option<FileId>;
}

fn find_packages(db: &dyn PackageDatabase) -> Vec<Package> {
    let mut packages = Vec::new();
    find_packages_recursive(db, db.root(), &mut packages);
    packages
}

fn find_packages_recursive(db: &dyn PackageDatabase, current: PathSegment, packages: &mut Vec<Package>) {
    let directory_data = db.directory_data(current);

    for directory in directory_data.directories() {
        find_packages_recursive(db, *directory, packages);
    }

    for file in directory_data.files() {
        let file_data = db.lookup_file_data(*file);
        if file_data.name == PACKAGE_MANIFEST_NAME {
            packages.push(db.package(*file));
        }
    }
}

fn package(db: &dyn PackageDatabase, file: FileId) -> Package {
    let contents = db.file_contents(file);
    // TODO handle invalid manifests better
    let manifest = Manifest::from_str(&contents).unwrap_or_else(|_| Manifest::empty());
    Package::new(file, Arc::new(manifest))
}

fn source_root(db: &dyn PackageDatabase, file: FileId) -> Option<FileId> {
    let package = db.package(file);
    if let Some(_path) = &package.manifest.package.path {
        unimplemented!("custom source paths in package manifests are not yet supported");
    } else {
        let target = db.directory(file);
        let target = db.child_directory(target, "src".to_owned())?;
        let target = db.child_file(target, "lib.skysl".to_owned())?;
        Some(target)
    }
}
