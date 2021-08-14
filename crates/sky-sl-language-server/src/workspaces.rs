use sky_sl::workspace::*;
use camino::{Utf8Path, Utf8PathBuf};
use dashmap::{DashMap, mapref::one::RefMut};
use std::sync::Mutex;

pub struct Workspaces {
    inner: DashMap<Utf8PathBuf, Mutex<Workspace>>,
}

impl Workspaces {
    pub fn new() -> Self {
        Self {
            inner: DashMap::new(),
        }
    }

    pub fn find_or_create(&self, query: &Utf8Path) -> Result<RefMut<'_, Utf8PathBuf, Mutex<Workspace>>, WorkspaceError> {
        let mut package_root = find_package_root(query).ok_or_else(|| WorkspaceError::NoPackageRootFound)?;
        package_root.push("skysl.toml");
        Ok(self.inner.entry(package_root.clone()).or_insert_with(|| Mutex::new(bootstrap(&package_root).unwrap())))
    }
}

pub fn find_package_root(mut query: &Utf8Path) -> Option<Utf8PathBuf> {
    if !query.is_absolute() {
        return None;
    }

    while !query.is_dir() {
        query = query.parent()?;
    }

    find_package_root_recursive(query)
}

fn find_package_root_recursive(query: &Utf8Path) -> Option<Utf8PathBuf> {
    let package_query = query.join("skysl.toml");
    if package_query.exists() {
        return Some(query.into());
    }

    find_package_root_recursive(query.parent()?)
}
