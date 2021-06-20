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
        let package_root = Workspace::find_package_root(query)?;
        Ok(self.inner.entry(package_root.clone()).or_insert_with(|| Workspace::new(package_root)))
    }
}
