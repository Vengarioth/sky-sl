use crate::vfs::{FileEvent, VirtualFileSystem};
use camino::{Utf8Path, Utf8PathBuf};
use sky_sl::workspace::Workspace;
use std::collections::HashMap;
use tower_lsp::lsp_types::*;

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

    pub fn synchronize(&mut self) {
        for change in self.vfs.changes() {
            match change {
                FileEvent::Created { path, contents } => {
                    self.workspace.insert_file(&path, contents).unwrap();
                }
                FileEvent::Changed {
                    path, new_contents, ..
                } => {
                    self.workspace.insert_file(&path, new_contents).unwrap();
                }
                FileEvent::Deleted { path, .. } => {
                    self.workspace.remove_file(&path).unwrap();
                }
                FileEvent::Opened { path, contents } => {
                    self.workspace.insert_file(&path, contents).unwrap();
                }
                FileEvent::Closed { .. } => {}
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

    pub fn document_symbols(&mut self, path: Utf8PathBuf) -> Option<DocumentSymbolResponse> {
        self.workspaces
            .values_mut()
            .find(|workspace| path.starts_with(workspace.root()))
            .map(|workspace| {
                workspace.synchronize();
                let ast = workspace.workspace.get_ast(&path).unwrap().tree();
                let line_index = workspace.workspace.get_line_index(&path).unwrap();
                crate::queries::document_symbols(ast, line_index)
            })
    }

    pub fn semantic_tokens(&mut self, path: Utf8PathBuf) -> Option<SemanticTokensResult> {
        self.workspaces
            .values_mut()
            .find(|workspace| path.starts_with(workspace.root()))
            .map(|workspace| {
                workspace.synchronize();
                let ast = workspace.workspace.get_ast(&path).unwrap().tree();
                let line_index = workspace.workspace.get_line_index(&path).unwrap();
                let tokens = crate::semantics::get_semantic_tokens(ast, &line_index);
                SemanticTokensResult::Tokens(tokens)
            })
    }

    pub fn created(&mut self, path: Utf8PathBuf) {
        self.workspaces.values_mut().for_each(|workspace| {
            if path.starts_with(workspace.root()) {
                workspace.vfs.create_file(path.clone()).unwrap();
            }
        })
    }

    pub fn deleted(&mut self, path: Utf8PathBuf) {
        self.workspaces.iter_mut().for_each(|(_, workspace)| {
            if path.starts_with(workspace.root()) {
                workspace.vfs.delete_file(path.clone()).unwrap();
            }
        })
    }

    pub fn did_open(&mut self, path: Utf8PathBuf, contents: String) {
        self.workspaces.iter_mut().for_each(|(_, workspace)| {
            if path.starts_with(workspace.root()) {
                workspace
                    .vfs
                    .open_file(path.clone(), contents.clone())
                    .unwrap();
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
                workspace
                    .vfs
                    .change_file_open(path.clone(), contents.clone())
                    .unwrap();
            }
        });
    }
}
