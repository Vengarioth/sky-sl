use camino::Utf8PathBuf;
use serde_json::Value;
use sky_sl::syn::ast::*;
use std::sync::Arc;
use tower_lsp::jsonrpc::*;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

mod semantics;
mod workspaces;

use workspaces::Workspaces;

struct Backend {
    client: Client,
    workspaces: Workspaces,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::Full,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".".to_string()]),
                    work_done_progress_options: Default::default(),
                    all_commit_characters: None,
                }),
                execute_command_provider: Some(ExecuteCommandOptions {
                    commands: vec!["dummy.do_something".to_string()],
                    work_done_progress_options: Default::default(),
                }),
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
                            legend: semantics::get_legend(),
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

    async fn initialized(&self, _params: InitializedParams) {
        self.client
            .log_message(MessageType::Info, "initialized!")
            .await;
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let path = url_to_path(&params.text_document.uri)?;
        let workspace = self
            .workspaces
            .find_or_create(&path)
            .map_err(|_| Error::new(ErrorCode::ServerError(1)))?;
        let file_path = path
            .strip_prefix(workspace.package_root())
            .map_err(|_| Error::new(ErrorCode::ServerError(1)))?;

        let mut symbols = Vec::new();
        let ast = workspace.ast(&file_path).unwrap().tree();
        let line_index = workspace.line_index(&file_path).unwrap();

        for struct_definition in ast.struct_definitions() {
            let syntax = struct_definition.syntax();
            let range = syntax.text_range();
            let start = line_index.find_position(range.start());
            let end = line_index.find_position(range.end());
            let range = Range::new(
                Position::new(start.line, start.column),
                Position::new(end.line, end.column),
            );

            if let Some(identifier) = struct_definition.identifier() {
                let selection_range = identifier.syntax().text_range();
                let start = line_index.find_position(selection_range.start());
                let end = line_index.find_position(selection_range.end());
                let selection_range = Range::new(
                    Position::new(start.line, start.column),
                    Position::new(end.line, end.column),
                );

                #[allow(deprecated)]
                let symbol = DocumentSymbol {
                    name: identifier.syntax().to_string(),
                    detail: None,
                    kind: SymbolKind::Struct,
                    tags: None,
                    range,
                    selection_range,
                    children: None,
                    deprecated: None,
                };
                symbols.push(symbol);
            }
        }

        for fn_definition in ast.function_definitions() {
            let syntax = fn_definition.syntax();
            let range = syntax.text_range();
            let start = line_index.find_position(range.start());
            let end = line_index.find_position(range.end());
            let range = Range::new(
                Position::new(start.line, start.column),
                Position::new(end.line, end.column),
            );

            if let Some(identifier) = fn_definition.identifier() {
                let selection_range = identifier.syntax().text_range();
                let start = line_index.find_position(selection_range.start());
                let end = line_index.find_position(selection_range.end());
                let selection_range = Range::new(
                    Position::new(start.line, start.column),
                    Position::new(end.line, end.column),
                );

                #[allow(deprecated)]
                let symbol = DocumentSymbol {
                    name: identifier.syntax().to_string(),
                    detail: None,
                    kind: SymbolKind::Function,
                    tags: None,
                    range,
                    selection_range,
                    children: None,
                    deprecated: None,
                };
                symbols.push(symbol);
            }
        }

        Ok(Some(DocumentSymbolResponse::Nested(symbols)))
    }

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        // See https://code.visualstudio.com/api/language-extensions/semantic-highlight-guide
        let path = url_to_path(&params.text_document.uri)?;
        let workspace = self
            .workspaces
            .find_or_create(&path)
            .map_err(|_| Error::new(ErrorCode::ServerError(1)))?;
        let file_path = path
            .strip_prefix(workspace.package_root())
            .map_err(|_| Error::new(ErrorCode::ServerError(1)))?;

        let root = workspace.ast(&file_path).unwrap().tree();
        let line_index = workspace.line_index(&file_path).unwrap();

        let semantic_tokens = semantics::get_semantic_tokens(root, &line_index);

        dbg!(&semantic_tokens);

        Ok(Some(SemanticTokensResult::Tokens(semantic_tokens)))
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_change_workspace_folders(&self, _params: DidChangeWorkspaceFoldersParams) {
        self.client
            .log_message(MessageType::Info, "workspace folders changed!")
            .await;
    }

    async fn did_change_configuration(&self, _: DidChangeConfigurationParams) {
        self.client
            .log_message(MessageType::Info, "configuration changed!")
            .await;
    }

    async fn did_change_watched_files(&self, _: DidChangeWatchedFilesParams) {
        self.client
            .log_message(MessageType::Info, "watched files have changed!")
            .await;
    }

    async fn execute_command(&self, _: ExecuteCommandParams) -> Result<Option<Value>> {
        self.client
            .log_message(MessageType::Info, "command executed!")
            .await;

        match self.client.apply_edit(WorkspaceEdit::default()).await {
            Ok(res) if res.applied => self.client.log_message(MessageType::Info, "applied").await,
            Ok(_) => self.client.log_message(MessageType::Info, "rejected").await,
            Err(err) => self.client.log_message(MessageType::Error, err).await,
        }

        Ok(None)
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let path = url_to_path(&params.text_document.uri).unwrap();
        let mut workspace = self.workspaces.find_or_create(&path).unwrap();
        let file_path = path.strip_prefix(workspace.package_root()).unwrap();

        workspace.update_file(
            file_path.into(),
            Arc::new(params.text_document.text.to_string()),
        );

        self.client
            .log_message(MessageType::Info, "file opened!")
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let version = params.text_document.version;
        let uri = params.text_document.uri;

        let path = url_to_path(&uri).unwrap();
        let mut workspace = self.workspaces.find_or_create(&path).unwrap();
        let file_path = path.strip_prefix(workspace.package_root()).unwrap();

        workspace.update_file(
            file_path.into(),
            Arc::new(params.content_changes[0].text.to_string()),
        );

        let ast = workspace.ast(file_path.into()).unwrap();
        let line_index = workspace.line_index(file_path.into()).unwrap();

        let mut diagnostics = Vec::new();
        for error in ast.errors() {
            dbg!(error);
            let start = line_index.find_position((error.offset as u32).into());
            let end = line_index.find_position(((error.offset + error.length) as u32).into());

            diagnostics.push(Diagnostic::new(
                Range::new(
                    Position::new(start.line, start.column),
                    Position::new(end.line, end.column),
                ),
                Some(DiagnosticSeverity::Error),
                Some(NumberOrString::Number(0)),
                Some(" ".to_string()),
                "Some Diagnostic".to_string(),
                None,
                None,
            ));
        }

        self.client
            .publish_diagnostics(uri, diagnostics, Some(version))
            .await;

        self.client
            .log_message(MessageType::Info, "file changed!")
            .await;
    }

    async fn did_save(&self, _: DidSaveTextDocumentParams) {
        self.client
            .log_message(MessageType::Info, "file saved!")
            .await;
    }

    async fn did_close(&self, _params: DidCloseTextDocumentParams) {
        self.client
            .log_message(MessageType::Info, "file closed!")
            .await;
    }

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        Ok(Some(CompletionResponse::Array(vec![
            CompletionItem::new_simple("Hello".to_string(), "Some detail".to_string()),
            CompletionItem::new_simple("Bye".to_string(), "More detail".to_string()),
        ])))
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, messages) = LspService::new(|client| Backend {
        client,
        workspaces: Workspaces::new(),
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
