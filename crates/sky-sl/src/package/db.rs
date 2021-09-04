use super::{Package, Manifest};
use crate::fs::{FileDatabase, PathSegment};
use std::str::FromStr;

#[salsa::query_group(PackageDatabaseStorage)]
pub trait PackageDatabase: FileDatabase {
    fn find_packages(&self) -> Vec<Package>;
    fn package(&self, path: PathSegment) -> Package;
    fn source_root(&self, package_path: PathSegment) -> PathSegment;
}

fn find_packages(db: &dyn PackageDatabase) -> Vec<Package> {
    db.files().iter().filter(|path_segment| {
        let file_info = db.file_info(**path_segment);
        file_info.name == "skysl.toml"
    }).map(|path_segment| {
        db.package(*path_segment)
    }).collect()
}

fn package(db: &dyn PackageDatabase, path: PathSegment) -> Package {
    let contents = db.file_contents(path);
    // TODO handle invalid manifests better
    let manifest = Manifest::from_str(&contents).unwrap_or_else(|_| Manifest::empty());
    Package::new(path, manifest)
}

fn source_root(db: &dyn PackageDatabase, package_path: PathSegment) -> PathSegment {
    let package = db.package(package_path);
    if let Some(_path) = package.manifest.package.path {
        unimplemented!("custom source paths in package manifests are not yet supported");
    } else {
        let target = db.parent_segment(package_path);
        let target = db.child_directory(target, "src".to_owned());
        let target = db.child_file(target, "lib.skysl".to_owned());
        target
    }
}
