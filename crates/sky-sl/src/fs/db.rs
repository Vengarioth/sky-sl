use super::{DirectoryData, PathSegment, PathSegmentData};
use std::sync::Arc;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FileContents {
    File(Arc<String>),
    NotFound,
}

#[salsa::query_group(FileDatabaseStorage)]
pub trait FileDatabase: salsa::Database {
    #[salsa::input]
    fn file_contents(&self, path: PathSegment) -> Arc<String>;

    fn find_file_contents(&self, path: PathSegment) -> FileContents;

    #[salsa::interned]
    fn intern_path_data(&self, data: PathSegmentData) -> PathSegment;

    #[salsa::input]
    fn directory_data(&self, path: PathSegment) -> DirectoryData;
}

fn find_file_contents(db: &dyn FileDatabase, path: PathSegment) -> FileContents {
    if let Some(contents) = find_file_contents_recursive(db, path) {
        FileContents::File(contents)
    } else {
        FileContents::NotFound
    }
}

fn find_file_contents_recursive(db: &dyn FileDatabase, path: PathSegment) -> Option<Arc<String>> {
    todo!()
}

fn set_file_contents(db: &mut dyn FileDatabase, path: PathSegment, contents: Arc<String>) {
    update_directory_data_recursive(db, path);
}

fn update_directory_data_recursive(db: &mut dyn FileDatabase, path: PathSegment) {
    match db.lookup_intern_path_data(path) {
        PathSegmentData::Directory { parent, .. } => {
            db.set_directory_data(parent, db.directory_data(parent).with_segment(path));
            update_directory_data_recursive(db, parent);
        },
        PathSegmentData::File { parent, .. } => {
            db.set_directory_data(parent, db.directory_data(parent).with_segment(path));
            update_directory_data_recursive(db, parent);
        }
        PathSegmentData::Root => {},
    }
}
