use camino::{Utf8Path, Utf8PathBuf};

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
