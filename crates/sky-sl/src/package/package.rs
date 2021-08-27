use super::PackageManifest;
use camino::Utf8PathBuf;

#[derive(Debug)]
pub struct Package {
    manifest_path: Utf8PathBuf,
    source_path: Utf8PathBuf,

    manifest: PackageManifest,
}
