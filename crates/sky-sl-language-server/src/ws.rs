use crate::vfs::{VirtualFileSystem, FileEvent};
use camino::{Utf8Path, Utf8PathBuf};
use sky_sl::ws::Workspace;
use std::collections::HashMap;

pub struct VSCodeWorkspace {
    workspace: Workspace,
    vfs: VirtualFileSystem,
}

impl VSCodeWorkspace {
    pub fn create(path: Utf8PathBuf) -> Self {
        Self {
            workspace: Workspace::create(path.clone()),
            vfs: VirtualFileSystem::create(path).expect("could not create virtual file system"),
        }
    }

    pub fn root(&self) -> &Utf8Path {
        self.vfs.root()
    }

    pub fn sync_fs(&mut self) {
        for change in self.vfs.changes() {
            match change {
                FileEvent::Created { path, contents } => self.workspace.insert_file(&path, contents),
                FileEvent::Changed { path, new_contents, .. } => self.workspace.insert_file(&path, new_contents),
                FileEvent::Deleted { path, .. } => self.workspace.remove_file(&path),
                FileEvent::Opened { .. } => (),
                FileEvent::Closed { .. } => (),
            }
        }
    }
}

pub struct VSCodeWorkspaces {
    workspaces: HashMap<String, VSCodeWorkspace>,
}

impl VSCodeWorkspaces {
    pub fn new() -> Self {
        Self {
            workspaces: HashMap::new(),
        }
    }

    pub fn create_workspace(&mut self, name: String, path: Utf8PathBuf) {
        self.workspaces.insert(name, VSCodeWorkspace::create(path));
    }

    pub fn did_open(&mut self, path: Utf8PathBuf) {
        self.workspaces.iter_mut().for_each(|(_, workspace)| {
            if path.starts_with(workspace.root()) {
                workspace.vfs.open_file(path.clone()).unwrap();
            }
        });
    }

    pub fn did_close(&mut self, path: Utf8PathBuf) {
        self.workspaces.iter_mut().for_each(|(_, workspace)| {
            if path.starts_with(workspace.root()) {
                workspace.vfs.close_file(&path).unwrap();
            }
        });
    }

    pub fn did_change(&mut self, path: Utf8PathBuf) {
        self.workspaces.iter_mut().for_each(|(_, workspace)| {
            if path.starts_with(workspace.root()) {
                workspace.vfs.change_file(path.clone()).unwrap();
            }
        });
    }

    pub fn did_change_open(&mut self, path: Utf8PathBuf, contents: String) {
        self.workspaces.iter_mut().for_each(|(_, workspace)| {
            if path.starts_with(workspace.root()) {
                workspace.vfs.change_file_open(path.clone(), contents.clone()).unwrap();
            }
        });
    }
}
