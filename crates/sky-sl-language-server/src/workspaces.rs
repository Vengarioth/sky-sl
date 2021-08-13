use sky_sl::workspace::*;
use camino::{Utf8Path, Utf8PathBuf};
use dashmap::{DashMap, mapref::one::RefMut};

pub struct Workspaces {
    inner: DashMap<Utf8PathBuf, Workspace>,
}

impl Workspaces {
    pub fn new() -> Self {
        Self {
            inner: DashMap::new(),
        }
    }

    pub fn find_or_create(&self, query: &Utf8Path) -> Result<RefMut<'_, Utf8PathBuf, Workspace>, WorkspaceError> {
        let package_root = find_package_root(query).ok_or_else(|| WorkspaceError::NoPackageRootFound)?;
        Ok(self.inner.entry(package_root.clone()).or_insert_with(|| Workspace::new(package_root)))
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
