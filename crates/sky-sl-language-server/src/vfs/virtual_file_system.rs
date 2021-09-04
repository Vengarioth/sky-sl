use super::VirtualFileSystemError;
use camino::{Utf8Path, Utf8PathBuf};
use glob::*;
use std::collections::{HashSet, VecDeque};
use std::sync::Arc;

#[derive(Debug)]
pub struct FileInfo {
    path: Utf8PathBuf,
    contents: Arc<String>,
}

#[derive(Debug)]
pub enum FileEvent {
    Created {
        path: Utf8PathBuf,
        contents: Arc<String>,
    },

    Changed {
        path: Utf8PathBuf,
        previous_contents: Arc<String>,
        new_contents: Arc<String>,
    },

    Deleted {
        path: Utf8PathBuf,
        previous_contents: Arc<String>,
    },

    Opened {
        path: Utf8PathBuf,
        contents: Arc<String>,
    },

    Closed {
        path: Utf8PathBuf,
        contents: Arc<String>,
    }
}

#[derive(Debug)]
pub struct VirtualFileSystem {
    root: Utf8PathBuf,
    files: Vec<FileInfo>,
    events: VecDeque<FileEvent>,
    open_files: HashSet<Utf8PathBuf>,
}

impl VirtualFileSystem {
    pub fn create(root: Utf8PathBuf) -> Result<Self, VirtualFileSystemError> {
        if !root.is_dir() {
            return Err(VirtualFileSystemError::RootIsNotADir(root));
        }

        if !root.is_absolute() {
            return Err(VirtualFileSystemError::RootIsNotAbsolute(root));
        }

        let mut files = Vec::new();
        let mut events = VecDeque::new();
        
        // construct glob pattern looking for all .skysl files in root
        let mut query = root.clone();
        query.push("**");
        query.push("*.skysl");
        let query = query.to_string();

        for result in glob(&query).map_err(|_| VirtualFileSystemError::InvalidRoot(root.clone()))? {
            let entry = result.map_err(|e| VirtualFileSystemError::CannotAccessFile(Utf8PathBuf::from_path_buf(e.path().to_owned()).expect("Could not convert path to utf8 path")))?;
            let entry = Utf8PathBuf::from_path_buf(entry).expect("Could not convert path to utf8 path");

            let contents = std::fs::read_to_string(&entry).map_err(|_| VirtualFileSystemError::CannotAccessFile(entry.clone()))?;
            let contents = Arc::new(contents);

            files.push(FileInfo {
                path: entry.clone(),
                contents: Arc::clone(&contents),
            });

            events.push_back(FileEvent::Created {
                path: entry,
                contents: contents,
            });
        }

        // construct glob pattern looking for all skysl.toml files in root
        let mut query = root.clone();
        query.push("**");
        query.push("skysl.toml");
        let query = query.to_string();

        for result in glob(&query).map_err(|_| VirtualFileSystemError::InvalidRoot(root.clone()))? {
            let entry = result.map_err(|e| VirtualFileSystemError::CannotAccessFile(Utf8PathBuf::from_path_buf(e.path().to_owned()).expect("Could not convert path to utf8 path")))?;
            let entry = Utf8PathBuf::from_path_buf(entry).expect("Could not convert path to utf8 path");

            let contents = std::fs::read_to_string(&entry).map_err(|_| VirtualFileSystemError::CannotAccessFile(entry.clone()))?;
            let contents = Arc::new(contents);

            files.push(FileInfo {
                path: entry.clone(),
                contents: Arc::clone(&contents),
            });

            events.push_back(FileEvent::Created {
                path: entry,
                contents: contents,
            });
        }

        Ok(Self {
            root,
            files,
            events,
            open_files: HashSet::new(),
        })
    }

    pub fn root(&self) -> &Utf8Path {
        &self.root
    }

    pub fn exists(&self, path: &Utf8Path) -> bool {
        self.files.iter().any(|file| file.path == path)
    }

    pub fn create_file(&mut self, path: Utf8PathBuf) -> Result<(), VirtualFileSystemError> {
        if self.exists(&path) {
            return Err(VirtualFileSystemError::FileAlreadyExists(path));
        }

        let contents = std::fs::read_to_string(&path).map_err(|_| VirtualFileSystemError::CannotAccessFile(path.clone()))?;
        let contents = Arc::new(contents);

        self.files.push(FileInfo {
            path: path.clone(),
            contents: Arc::clone(&contents),
        });

        self.events.push_back(FileEvent::Created {
            path: path,
            contents: contents,
        });

        Ok(())
    }

    pub fn change_file(&mut self, path: Utf8PathBuf) -> Result<(), VirtualFileSystemError> {
        if let Some(index) = self.files.iter().position(|file| file.path == path) {
            let previous_contents = Arc::clone(&self.files[index].contents);

            let contents = std::fs::read_to_string(&path).map_err(|_| VirtualFileSystemError::CannotAccessFile(path.clone()))?;
            let contents = Arc::new(contents);

            self.files[index] = FileInfo {
                path: path.clone(),
                contents: Arc::clone(&contents),
            };
            self.events.push_back(FileEvent::Changed {
                path: path,
                previous_contents,
                new_contents: contents,
            });

            Ok(())
        } else {
            Err(VirtualFileSystemError::FileDoesNotExist(path))
        }
    }

    pub fn change_file_open(&mut self, path: Utf8PathBuf, contents: String) -> Result<(), VirtualFileSystemError> {
        if let Some(index) = self.files.iter().position(|file| file.path == path) {
            let previous_contents = Arc::clone(&self.files[index].contents);

            let contents = Arc::new(contents);

            self.files[index] = FileInfo {
                path: path.clone(),
                contents: Arc::clone(&contents),
            };
            self.events.push_back(FileEvent::Changed {
                path: path,
                previous_contents,
                new_contents: contents,
            });

            Ok(())
        } else {
            Err(VirtualFileSystemError::FileDoesNotExist(path))
        }
    }

    pub fn delete_file(&mut self, path: Utf8PathBuf) -> Result<(), VirtualFileSystemError> {
        if let Some(index) = self.files.iter().position(|file| file.path == path) {
            let file = self.files.remove(index);

            self.events.push_back(FileEvent::Deleted {
                path,
                previous_contents: file.contents,
            });

            Ok(())
        } else {
            Err(VirtualFileSystemError::FileDoesNotExist(path))
        }
    }

    pub fn open_file(&mut self, path: Utf8PathBuf) -> Result<(), VirtualFileSystemError> {
        self.open_files.insert(path);
        Ok(())
    }

    pub fn is_file_open(&self, path: &Utf8Path) -> Result<bool, VirtualFileSystemError> {
        Ok(self.open_files.contains(path))
    }

    pub fn close_file(&mut self, path: &Utf8Path) -> Result<(), VirtualFileSystemError> {
        self.open_files.remove(path);
        Ok(())
    }

    pub fn changes(&mut self) -> FileEventIter {
        FileEventIter::new(self)
    }
}

pub struct FileEventIter<'a> {
    vfs: &'a mut VirtualFileSystem,
}

impl<'a> FileEventIter<'a> {
    fn new(vfs: &'a mut VirtualFileSystem) -> Self {
        Self {
            vfs,
        }
    }
}

impl<'a> Iterator for FileEventIter<'a> {
    type Item = FileEvent;

    fn next(&mut self) -> Option<Self::Item> {
        self.vfs.events.pop_front()
    }
}
