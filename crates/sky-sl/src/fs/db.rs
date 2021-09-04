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

    fn files(&self) -> Vec<PathSegment>;

    fn file_info(&self, path: PathSegment) -> FileInfo;

    fn parent_segment(&self, path: PathSegment) -> PathSegment;

    fn child_directory(&self, path: PathSegment, child_name: String) -> PathSegment;

    fn child_file(&self, path: PathSegment, child_name: String) -> PathSegment;
}

fn parent_segment(db: &dyn FileDatabase, path: PathSegment) -> PathSegment {
    path.parent(db).unwrap()
}

fn child_directory(db: &dyn FileDatabase, path: PathSegment, directory_name: String) -> PathSegment {
    db.intern_path_data(PathSegmentData::Directory {
        name: directory_name,
        parent: path,
    })
}

fn child_file(db: &dyn FileDatabase, path: PathSegment, file_name: String) -> PathSegment {
    db.intern_path_data(PathSegmentData::File {
        name: file_name,
        parent: path,
    })
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FileInfo {
    pub name: String,
}

fn file_info(db: &dyn FileDatabase, path: PathSegment) -> FileInfo {
    let path_data = db.lookup_intern_path_data(path);
    match path_data {
        PathSegmentData::Root => panic!("path was a root, not a file"),
        PathSegmentData::Directory { .. } => panic!("path was a directory, not a file"),
        PathSegmentData::File { name, .. } => FileInfo {
            name: name.to_owned(),
        }
    }
}

fn files(db: &dyn FileDatabase) -> Vec<PathSegment> {
    let mut files = Vec::new();
    let mut stack = Vec::new();
    stack.push(db.intern_path_data(PathSegmentData::Root));

    while let Some(directory) = stack.pop() {
        let directory_data = db.directory_data(directory);

        for item in directory_data.iter() {
            let item_data = db.lookup_intern_path_data(*item);
            if item_data.is_file() {
                files.push(*item);
            } else {
                stack.push(*item);
            }
        }
    }

    files
}

pub fn initialize_fs(db: &mut dyn FileDatabase) {
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

pub fn remove_file(db: &mut dyn FileDatabase, path: &Utf8Path) {
    let root = db.intern_path_data(PathSegmentData::Root);
    let mut current = root;

    // recurse through the hierachy until we find the correct PathSegment that holds the file, or return early
    for component in path.components() {
        let parent = current;
        let parent_directory_data = db.directory_data(parent);
    
        if let Some(next) = parent_directory_data.find_by_name(component.as_str(), db) {
            current = next;
        } else {
            // component is not part of the hierachy
            return;
        }
    }

    // walk backwards up the hierachy and remove links where apropriate
    while let Some(parent) = current.parent(db) {
        let parent_data =  db.directory_data(parent).without_segment(current);
        let remaining_children = parent_data.len();
        db.set_directory_data(parent, parent_data);

        // only remove links until other children hold the connection
        if remaining_children != 0 {
            return;
        }

        current = parent;
    }
}

pub fn check_consistency(db: &dyn FileDatabase) {
    let root = db.intern_path_data(PathSegmentData::Root);
    let root_data = db.directory_data(root);
    for child in root_data.iter() {
        check_consistency_recursive(db, *child, root);
    }
}

fn check_consistency_recursive(db: &dyn FileDatabase, path: PathSegment, real_parent: PathSegment) {
    match db.lookup_intern_path_data(path) {
        PathSegmentData::Root => {
            panic!("multiple roots found");
        },
        PathSegmentData::Directory { parent, name } => {
            if parent != real_parent {
                panic!("parents do not match");
            }

            for child in db.directory_data(path).iter() {
                check_consistency_recursive(db, *child, path);
            }

            println!("{} is consistent", name);
        },
        PathSegmentData::File { parent, name } => {
            if parent != real_parent {
                panic!("parents do not match");
            }

            println!("{} is consistent", name);
        },
    }
}

#[derive(Debug)]
pub enum DebugItem {
    Root { children: Vec<DebugItem> },
    Directory { name: String, children: Vec<DebugItem> },
    File { name: String },
}

/// Prints the directory structure
pub fn debug(db: &dyn FileDatabase) -> DebugItem {
    let root = db.intern_path_data(PathSegmentData::Root);
    debug_recursive(db, root)
}

/// recursively walks the directory structure and prints names indented
fn debug_recursive(db: &dyn FileDatabase, path: PathSegment) -> DebugItem {
    match db.lookup_intern_path_data(path) {
        PathSegmentData::Root => {
            DebugItem::Root {
                children: debug_recurse_directory(db, db.directory_data(path)),
            }
        },
        PathSegmentData::Directory { name, .. } => {
            DebugItem::Directory {
                name: name.to_owned(),
                children: debug_recurse_directory(db, db.directory_data(path)),
            }
        },
        PathSegmentData::File { name, .. } => {
            DebugItem::File { name }
        },
    }
}

fn debug_recurse_directory(db: &dyn FileDatabase, directory_data: DirectoryData) -> Vec<DebugItem> {
    directory_data.iter().map(|child| debug_recursive(db, *child)).collect()
}
