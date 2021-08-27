use super::vfs::*;
use sky_sl::workspace::Workspace;
use camino::{Utf8Path, Utf8PathBuf};
use std::sync::Arc;

struct OpenWorkspace {
    file_system: VirtualFileSystem,
    workspace: Workspace,
}

impl OpenWorkspace {
    pub fn before_query(&mut self) {
        for change in self.file_system.changes() {
            match change {
                FileEvent::Created { path, contents } => {
                    self.workspace.set_file_contents(path, contents);
                },
                FileEvent::Changed { path, new_contents, .. } => {
                    self.workspace.set_file_contents(path, new_contents);
                },
                FileEvent::Deleted { path, .. } => {
                    // for now we just set an empty string
                    // this could be better represented by a file struct with an Option<String> for the contents
                    // see https://github.com/salsa-rs/salsa/issues/37
                    self.workspace.set_file_contents(path, Arc::new(String::new()));
                },
                FileEvent::Opened { .. } => {
                    // idealy begin keeping syntax trees here
                },
                FileEvent::Closed { .. } => {
                    // idealy throw away syntax trees here
                },
            }
        }
    }
}

pub struct Workspaces {
    workspaces: Vec<OpenWorkspace>,
}

impl Workspaces {
    pub fn new() -> Self {
        Self {
            workspaces: Vec::new(),
        }
    }

    pub fn create_workspace(&mut self, root: Utf8PathBuf) {
        let file_system = VirtualFileSystem::create(root.clone()).expect("could not create virtual file system");
        let workspace = Workspace::create(root).expect("could not create workspace");

        self.workspaces.push(OpenWorkspace {
            file_system,
            workspace,
        });
    }

    pub fn remove_workspace(&mut self, root: Utf8PathBuf) {
        self.workspaces.retain(|workspace| workspace.file_system.root() != root);
    }

    pub fn is_file_open(&self, path: &Utf8Path) -> bool {
        self.workspaces.iter().any(|workspace| workspace.file_system.is_file_open(path).unwrap())
    }

    pub fn file_created(&mut self, path: Utf8PathBuf) {
        for workspace in &mut self.workspaces {
            if path.starts_with(workspace.file_system.root()) {
                workspace.file_system.create_file(path.clone()).unwrap();
            }
        }
    }

    pub fn file_deleted(&mut self, path: Utf8PathBuf) {
        for workspace in &mut self.workspaces {
            if path.starts_with(workspace.file_system.root()) {
                workspace.file_system.delete_file(path.clone()).unwrap();
            }
        }
    }

    pub fn file_opened(&mut self, path: Utf8PathBuf) {
        for workspace in &mut self.workspaces {
            if path.starts_with(workspace.file_system.root()) {
                workspace.file_system.open_file(path.clone()).unwrap();
            }
        }
    }

    pub fn file_closed(&mut self, path: Utf8PathBuf) {
        for workspace in &mut self.workspaces {
            if path.starts_with(workspace.file_system.root()) {
                workspace.file_system.close_file(&path).unwrap();
            }
        }
    }

    pub fn file_changed(&mut self, path: Utf8PathBuf) {
        for workspace in &mut self.workspaces {
            if path.starts_with(workspace.file_system.root()) {
                workspace.file_system.change_file(path.clone()).unwrap();
            }
        }
    }

    pub fn open_file_changed(&mut self, path: Utf8PathBuf, contents: String) {
        for workspace in &mut self.workspaces {
            if path.starts_with(workspace.file_system.root()) {
                workspace.file_system.change_file_open(path.clone(), contents.clone()).unwrap();
            }
        }
    }

    pub fn get_semantic_tokens(&mut self, path: Utf8PathBuf) -> Option<()> {
        let workspace = self.workspaces.iter_mut().find(|workspace| path.starts_with(workspace.file_system.root()))?;
        workspace.before_query();

        None
    }
}
