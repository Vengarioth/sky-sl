use super::{Backend, lsp::*};
use tower_lsp::LanguageServer;
use tower_lsp::lsp_types::*;
use tower_lsp::jsonrpc::*;

impl Backend {
    fn server_info() -> ServerInfo {
        ServerInfo {
            name: env!("CARGO_PKG_VERSION").to_owned(),
            version: option_env!("CARGO_PKG_NAME").map(|version| version.to_owned()),
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        
        dbg!("initialize");

        let mut workspaces = self.workspaces.lock().unwrap();
        if let Some(workspace_folders) = &params.workspace_folders {
            for workspace_folder in workspace_folders {
                let path = url_to_path(&workspace_folder.uri)?;
                workspaces.create_workspace(path);
            }
        }

        Ok(InitializeResult {
            server_info: Some(Self::server_info()),
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::Full,
                )),
                workspace: Some(WorkspaceServerCapabilities {
                    workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                        supported: Some(true),
                        change_notifications: Some(OneOf::Left(true)),
                    }),
                    file_operations: None,
                }),
                semantic_tokens_provider: Some(
                    SemanticTokensServerCapabilities::SemanticTokensOptions(
                        SemanticTokensOptions {
                            legend: super::semantics::get_legend(),
                            full: Some(SemanticTokensFullOptions::Delta { delta: Some(true) }),
                            range: None,
                            work_done_progress_options: Default::default(),
                        },
                    ),
                ),
                ..ServerCapabilities::default()
            },
        })
    }

    async fn did_change_workspace_folders(&self, params: DidChangeWorkspaceFoldersParams) {
        let mut workspaces = self.workspaces.lock().unwrap();
        for added_workspace in &params.event.added {
            let root = url_to_path(&added_workspace.uri).expect("could not convert uri to path");
            workspaces.create_workspace(root);
        }

        for removed_workspace in &params.event.removed {
            let root = url_to_path(&removed_workspace.uri).expect("could not convert uri to path");
            workspaces.remove_workspace(root);
        }
        
        dbg!("did_change_workspace_folders");
    }

    async fn did_change_watched_files(&self, params: DidChangeWatchedFilesParams) {
        let mut workspaces = self.workspaces.lock().unwrap();
        for change in &params.changes {
            let path = url_to_path(&change.uri).expect("could not convert uri to path");
            match change.typ {
                FileChangeType::Created => workspaces.file_created(path),
                FileChangeType::Deleted => workspaces.file_deleted(path),
                FileChangeType::Changed => {
                    // only send file-watcher based notifications if the file is not open
                    if !workspaces.is_file_open(&path) {
                        workspaces.file_changed(path);
                    }
                },
            }
        }
        
        dbg!("did_change_watched_files");
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let mut workspaces = self.workspaces.lock().unwrap();
        let path = url_to_path(&params.text_document.uri).expect("could not convert uri to path");
        workspaces.file_opened(path);
        
        dbg!("did_open");
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let mut workspaces = self.workspaces.lock().unwrap();
        let path = url_to_path(&params.text_document.uri).expect("could not convert uri to path");
        workspaces.file_closed(path);

        dbg!("did_close");
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let mut workspaces = self.workspaces.lock().unwrap();
        let path = url_to_path(&params.text_document.uri).expect("could not convert uri to path");
        let contents = params.content_changes[0].text.clone();
        workspaces.open_file_changed(path, contents);

        dbg!("did_change");
    }

    async fn semantic_tokens_full(&self, params: SemanticTokensParams) -> Result<Option<SemanticTokensResult>> {
        let mut workspaces = self.workspaces.lock().unwrap();
        let path = url_to_path(&params.text_document.uri).expect("could not convert uri to path");
        workspaces.get_semantic_tokens(path);
        
        dbg!("semantic_tokens_full");
        Ok(None)
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}
