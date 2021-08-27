  
use super::tree::*;
use camino::Utf8Path;
use std::sync::Arc;

#[salsa::query_group(FileDatabaseStorage)]
pub trait FileDatabase: salsa::Database {
    #[salsa::input]
    fn file_contents(&self, path: PathSegment) -> Arc<String>;

    fn find_file_contents(&self, path: PathSegment) -> Option<Arc<String>>;

    #[salsa::interned]
    fn intern_path_data(&self, data: PathSegmentData) -> PathSegment;

    #[salsa::input]
    fn directory_data(&self, path: PathSegment) -> DirectoryData;
}

pub fn initialize(db: &mut dyn FileDatabase) {
    let root = db.intern_path_data(PathSegmentData::Root);
    db.set_directory_data(root, DirectoryData::new());
}

pub fn find_file_contents(db: &dyn FileDatabase, path: PathSegment) -> Option<Arc<String>> {
    // TODO this can be solved without allocating a Vec
    
    if !db.lookup_intern_path_data(path).is_file() {
        return None;
    }

    let mut current = path;
    let mut segments = vec![];
    while let Some(parent) = db.lookup_intern_path_data(current).parent() {
        segments.push((parent, current));
        current = parent;
    }

    for (parent, child) in segments.iter().rev() {
        let directory_data = db.directory_data(*parent);
        if !directory_data.contains(child) {
            return None;
        }
    }

    Some(db.file_contents(path))
}

pub fn insert_file(db: &mut dyn FileDatabase, path: &Utf8Path, contents: Arc<String>) -> PathSegment {
    let root = db.intern_path_data(PathSegmentData::Root);

    let mut current = root;
    let mut iter = path.components().peekable();

    while let Some(component) = iter.next() {
        let name = component.to_string();
        let parent = current;

        let parent_directory_data = db.directory_data(parent);

        if iter.peek().is_some() {
            current = db.intern_path_data(PathSegmentData::Directory {
                name,
                parent,
            });

            // initialize DirectoryData for new directories
            if !parent_directory_data.contains(&current) {
                db.set_directory_data(current, DirectoryData::new());
            }

        } else {
            current = db.intern_path_data(PathSegmentData::File {
                name,
                parent,
            });
        }

        if !parent_directory_data.contains(&current) {
            db.set_directory_data(parent, parent_directory_data.with_segment(current));
        }
    }

    db.set_file_contents(current, contents);

    current
}
