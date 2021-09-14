use camino::Utf8PathBuf;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tower_lsp::jsonrpc::*;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

mod queries;
mod vfs;
mod workspace;

use workspace::VSCodeWorkspaces;

#[derive(Debug, Deserialize, Serialize)]
struct SyntaxTreeParams {
    text_ocument: TextDocumentIdentifier,
    range: Option<Range>,
}

struct Backend {
    client: Client,
    vscode_workspaces: Mutex<VSCodeWorkspaces>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        let mut workspaces = self.vscode_workspaces.lock().unwrap();
        if let Some(workspace_folders) = &params.workspace_folders {
            for workspace_folder in workspace_folders {
                let root_path = url_to_path(&workspace_folder.uri).unwrap();
                workspaces.create_workspace(workspace_folder.name.to_string(), root_path);
            }
        }

        Ok(InitializeResult {
            server_info: None,
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
                document_symbol_provider: Some(OneOf::Left(true)),
                semantic_tokens_provider: Some(
                    SemanticTokensServerCapabilities::SemanticTokensOptions(
                        SemanticTokensOptions {
                            legend: crate::queries::get_legend(),
                            full: Some(SemanticTokensFullOptions::Delta { delta: Some(true) }),
                            range: None,
                            work_done_progress_options: Default::default(),
                        },
                    ),
                ),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, _params: InitializedParams) {
        self.client
            .log_message(MessageType::Info, "initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let path = url_to_path(&params.text_document_position_params.text_document.uri)?;
        let position = params.text_document_position_params.position;
        Ok(self.vscode_workspaces.lock().unwrap().hover(path, position))
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let path = url_to_path(&params.text_document.uri)?;
        Ok(self
            .vscode_workspaces
            .lock()
            .unwrap()
            .document_symbols(path))
    }

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        let path = url_to_path(&params.text_document.uri)?;
        Ok(self.vscode_workspaces.lock().unwrap().semantic_tokens(path))
    }

    async fn did_change_workspace_folders(&self, _params: DidChangeWorkspaceFoldersParams) {
        self.client
            .log_message(MessageType::Info, "workspace folders changed!")
            .await;
        todo!();
    }

    async fn did_change_configuration(&self, _: DidChangeConfigurationParams) {
        self.client
            .log_message(MessageType::Info, "configuration changed!")
            .await;
    }

    async fn did_change_watched_files(&self, params: DidChangeWatchedFilesParams) {
        // needs its own block to not be captured in the future
        {
            let mut workspaces = self.vscode_workspaces.lock().unwrap();
            for change in &params.changes {
                let path = url_to_path(&change.uri).unwrap();
                match change.typ {
                    FileChangeType::Created => workspaces.created(path),
                    FileChangeType::Changed => workspaces.did_change(path),
                    FileChangeType::Deleted => workspaces.deleted(path),
                }
            }
        }

        self.client
            .log_message(MessageType::Info, "watched files have changed!")
            .await;
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let path = url_to_path(&params.text_document.uri).unwrap();
        let contents = params.text_document.text.clone();
        self.vscode_workspaces
            .lock()
            .unwrap()
            .did_open(path, contents);

        self.client
            .log_message(MessageType::Info, "file opened!")
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let path = url_to_path(&params.text_document.uri).unwrap();
        let contents = params.content_changes[0].text.clone();
        self.vscode_workspaces
            .lock()
            .unwrap()
            .did_change_open(path, contents);

        self.client
            .log_message(MessageType::Info, "open file changed!")
            .await;
    }

    async fn did_save(&self, _: DidSaveTextDocumentParams) {
        self.client
            .log_message(MessageType::Info, "file saved!")
            .await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let path = url_to_path(&params.text_document.uri).unwrap();
        self.vscode_workspaces.lock().unwrap().did_close(path);
        self.client
            .log_message(MessageType::Info, "file closed!")
            .await;
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, messages) = LspService::new(|client| Backend {
        client,
        vscode_workspaces: Mutex::new(VSCodeWorkspaces::new()),
    });

    Server::new(stdin, stdout)
        .interleave(messages)
        .serve(service)
        .await;
}

fn url_to_path(url: &Url) -> Result<Utf8PathBuf> {
    let path = url.to_file_path().map_err(|_| Error {
        code: ErrorCode::InvalidParams,
        message: "Cannot convert Url to Path".to_string(),
        data: None,
    })?;

    Utf8PathBuf::from_path_buf(path).map_err(|_| Error {
        code: ErrorCode::InvalidParams,
        message: "Cannot convert Url to Path".to_string(),
        data: None,
    })
}
