use super::{file::*, tree::*, FileSystemError};
use camino::Utf8Path;
use std::sync::Arc;

#[salsa::query_group(FileDatabaseStorage)]
pub trait FileDatabase: salsa::Database {
    #[salsa::interned]
    fn path_data(&self, data: PathSegmentData) -> PathSegment;

    #[salsa::input]
    fn directory_data(&self, path: PathSegment) -> DirectoryData;

    /// Interns the FileData and returns a FileId. Use `lookup_file_data` to retrieve the FileData of a FileId.
    #[salsa::interned]
    fn file_data(&self, data: FileData) -> FileId;

    /// Returns the file contents for a given FileId. Use `get_file_contents` to retrieve the file contents.
    #[salsa::input]
    fn file_contents(&self, file_id: FileId) -> Arc<String>;

    /// Returns the file system root of the workspace
    fn root(&self) -> PathSegment;
    fn directory(&self, file: FileId) -> PathSegment;
    fn parent_directory(&self, path: PathSegment) -> Option<PathSegment>;
    fn child_directory(&self, path: PathSegment, name: String) -> Option<PathSegment>;
    fn child_file(&self, path: PathSegment, name: String) -> Option<FileId>;
}

fn directory(db: &dyn FileDatabase, file: FileId) -> PathSegment {
    db.lookup_file_data(file).parent
}

fn parent_directory(db: &dyn FileDatabase, path: PathSegment) -> Option<PathSegment> {
    match db.lookup_path_data(path) {
        PathSegmentData::Root => None,
        PathSegmentData::Directory { parent, .. } => Some(parent)
    }
}

fn child_directory(db: &dyn FileDatabase, path: PathSegment, name: String) -> Option<PathSegment> {
    let directory_data = db.directory_data(path);
    let query = db.path_data(PathSegmentData::Directory {
        parent: path,
        name: name.to_string(),
    });
    if directory_data.contains_directory(&query) {
        Some(query)
    } else {
        None
    }
}

fn child_file(db: &dyn FileDatabase, path: PathSegment, name: String) -> Option<FileId> {
    let directory_data = db.directory_data(path);
    let query = db.file_data(FileData::new(name, path));
    if directory_data.contains_file(&query) {
        Some(query)
    } else {
        None
    }
}

#[inline]
fn root(db: &dyn FileDatabase) -> PathSegment {
    db.path_data(PathSegmentData::Root)
}

pub fn initialize_fs(db: &mut dyn FileDatabase) {
    let root = db.root();
    db.set_directory_data(root, DirectoryData::new());
}

/// Inserts a file into the VFS
pub fn insert_file(
    db: &mut dyn FileDatabase,
    path: &Utf8Path,
    contents: Arc<String>,
) -> Result<FileId, FileSystemError> {
    let root = db.root();

    // recursively walk and create the directory structure
    let mut current = root;
    for component in path
        .parent()
        .ok_or_else(|| FileSystemError::IsNotAValidFilePath(path.to_owned()))?
        .components()
    {
        let name = component.to_string();
        let parent = current;

        let parent_directory_data = db.directory_data(parent);
        current = db.path_data(PathSegmentData::Directory { name, parent });

        if !parent_directory_data.contains_directory(&current) {
            db.set_directory_data(parent, parent_directory_data.with_directory(current));
            db.set_directory_data(current, DirectoryData::new());
        }
    }

    // create the file
    let name = path
        .file_name()
        .ok_or_else(|| FileSystemError::IsNotAValidFilePath(path.to_owned()))?;
    let file_id = db.file_data(FileData::new(name.to_owned(), current));

    let directory_data = db.directory_data(current);
    db.set_directory_data(current, directory_data.with_file(file_id));

    db.set_file_contents(file_id, contents);
    Ok(file_id)
}

pub fn lookup_file(db: &dyn FileDatabase, path: &Utf8Path) -> Option<FileId> {
    let root = db.root();

    let mut current = root;
    for component in path.parent()?.components() {
        let parent = current;
        let parent_directory_data = db.directory_data(parent);
        current = db.path_data(PathSegmentData::Directory {
            name: component.to_string(),
            parent,
        });

        if !parent_directory_data.contains_directory(&current) {
            return None;
        }
    }

    let name = path.file_name()?;
    let directory_data = db.directory_data(current);

    for file_id in directory_data.files() {
        if db.lookup_file_data(*file_id).name == name {
            return Some(*file_id);
        }
    }

    None
}

pub fn remove_file(db: &mut dyn FileDatabase, path: &Utf8Path) -> Result<(), FileSystemError> {
    let root = db.root();

    // recursively walk the path to reach the file, or error out
    let mut current = root;
    for component in path
        .parent()
        .ok_or_else(|| FileSystemError::IsNotAValidFilePath(path.to_owned()))?
        .components()
    {
        let parent = current;
        let parent_directory_data = db.directory_data(parent);
        current = db.path_data(PathSegmentData::Directory {
            name: component.to_string(),
            parent,
        });

        if !parent_directory_data.contains_directory(&current) {
            return Err(FileSystemError::FileDoesNotExist(path.to_owned()));
        }
    }

    let name = path
        .file_name()
        .ok_or_else(|| FileSystemError::IsNotAValidFilePath(path.to_owned()))?;
    let file_id = db.file_data(FileData::new(name.to_owned(), current));

    // TODO check if file_id was in current
    let directory_data = db.directory_data(current);
    if !directory_data.contains_file(&file_id) {
        return Err(FileSystemError::FileDoesNotExist(path.to_owned()));
    }

    db.set_directory_data(current, directory_data.without_file(file_id));

    // remove directories without contents
    while let Some(parent) = current.parent(db) {
        let parent_data = db.directory_data(parent).without_directory(current);
        let remaining_children = parent_data.directory_count();

        db.set_directory_data(parent, parent_data);

        // only remove links if no children are left
        if remaining_children != 0 {
            break;
        }

        current = parent;
    }

    Ok(())
}
