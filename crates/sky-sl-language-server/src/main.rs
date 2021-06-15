use std::str::FromStr;

use serde_json::Value;
use sky_sl::workspace::Workspace;
use tower_lsp::jsonrpc::*;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use camino::Utf8PathBuf;

mod semantics;

#[derive(Debug)]
struct Backend {
    client: Client,
    workspaces: Vec<Workspace>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::Incremental,
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
                semantic_tokens_provider: Some(SemanticTokensServerCapabilities::SemanticTokensOptions(SemanticTokensOptions {
                    legend: SemanticTokensLegend {
                        token_types: vec![SemanticTokenType::STRUCT, SemanticTokenType::FUNCTION],
                        token_modifiers: vec![SemanticTokenModifier::DOCUMENTATION, SemanticTokenModifier::DECLARATION],
                    },
                    full: Some(SemanticTokensFullOptions::Bool(true)),
                    range: None,
                    work_done_progress_options: Default::default(),
                })),
                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, params: InitializedParams) {
        dbg!(params);
        self.client
            .log_message(MessageType::Info, "initialized!")
            .await;
    }

    async fn document_symbol(&self, params: DocumentSymbolParams) -> Result<Option<DocumentSymbolResponse>> {
        dbg!(&params);

        let path = url_to_path(&params.text_document.uri)?;
        let workspace = Workspace::new(path.clone());
        let result = workspace.document_symbols(path.clone()).unwrap();
        let line_index = workspace.get_line_index(path).unwrap();

        let symbols = to_document_symbols(result.root, &line_index);

        Ok(Some(DocumentSymbolResponse::Nested(symbols)))
    }

    async fn semantic_tokens_full(&self, params: SemanticTokensParams) -> Result<Option<SemanticTokensResult>> {
        // See https://code.visualstudio.com/api/language-extensions/semantic-highlight-guide
        let path = url_to_path(&params.text_document.uri)?;
        let semantic_tokens = get_semantic_symbols(path);

        Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
            result_id: None,
            data: semantic_tokens,
        })))
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_change_workspace_folders(&self, params: DidChangeWorkspaceFoldersParams) {
        dbg!(params);
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
        dbg!(&params);
        match Utf8PathBuf::from_str(&params.text_document.uri.path()) {
            Err(e) => self.client.log_message(MessageType::Error, format!("Could not handle path: {}", e)).await,
            Ok(path) => {
                dbg!(path);
            },
        }

        self.client
            .log_message(MessageType::Info, "file opened!")
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        dbg!(params);

        self.client
            .log_message(MessageType::Info, "file changed!")
            .await;
    }

    async fn did_save(&self, _: DidSaveTextDocumentParams) {
        self.client
            .log_message(MessageType::Info, "file saved!")
            .await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        dbg!(&params);
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
        workspaces: Vec::new(),
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

fn get_semantic_symbols(path: Utf8PathBuf) -> Vec<SemanticToken> {
    use sky_sl::syn::cst::*;

    let workspace = sky_sl::workspace::Workspace::new(path.clone());
    let result = workspace.document_symbols(path.clone()).unwrap();
    let line_index = workspace.get_line_index(path).unwrap();
    let mut semantic_tokens = Vec::new();
    for event in result.root.preorder_with_tokens() {
        match event {
            WalkEvent::Enter(element) => {
                match element {
                    SyntaxElement::Node(_) => {
                    },
                    SyntaxElement::Token(token) => {
                        let range = token.text_range();
                        let start = line_index.find_position(range.start());

                        semantic_tokens.push(SemanticToken {
                            delta_line: start.line,
                            delta_start: start.column,
                            length: range.len().into(),
                            token_type: 0,
                            token_modifiers_bitset: 0,
                        });

                    },
                }
            },
            WalkEvent::Leave(_) => {

            },
        }
    }

    semantic_tokens
}

fn to_document_symbols(node: sky_sl::syn::cst::SyntaxNode, line_index: &sky_sl::syn::cst::LineIndex) -> Vec<DocumentSymbol> {
    use sky_sl::syn::cst::*;
    // TODO refactor to use AST
    
    let mut stack: Vec<Vec<DocumentSymbol>> = Vec::new();
    stack.push(Vec::new());

    for event in node.preorder_with_tokens() {
        match event {
            WalkEvent::Enter(element) => {
                match element {
                    SyntaxElement::Node(_) => {
                        stack.push(Vec::new());
                    },
                    SyntaxElement::Token(token) => {
                        let range = token.text_range();
                        let start = line_index.find_position(range.start());
                        let end = line_index.find_position(range.end());
                        let range = Range::new(
                            Position::new(start.line, start.column),
                            Position::new(end.line, end.column),
                        );
                        
                        #[allow(deprecated)]
                        let symbol = DocumentSymbol {
                            name: "Token".to_string(),
                            detail: None,
                            kind: convert_syntax_kind(token.kind()),
                            tags: None,
                            range: range,
                            selection_range: range,
                            children: None,
                            deprecated: None,
                        };

                        stack.last_mut().map(|symbols| symbols.push(symbol));
                    },
                }
            },
            WalkEvent::Leave(element) => {
                match element {
                    SyntaxElement::Node(node) => {
                        let range = node.text_range();
                        let start = line_index.find_position(range.start());
                        let end = line_index.find_position(range.end());
                        let range = Range::new(
                            Position::new(start.line, start.column),
                            Position::new(end.line, end.column),
                        );

                        let children = stack.pop().unwrap();

                        #[allow(deprecated)]
                        let symbol = DocumentSymbol {
                            name: "Token".to_string(),
                            detail: None,
                            kind: convert_syntax_kind(node.kind()),
                            tags: None,
                            range: range,
                            selection_range: range,
                            children: Some(children),
                            deprecated: None,
                        };

                        stack.last_mut().map(|symbols| symbols.push(symbol));
                    },
                    SyntaxElement::Token(_) => {},
                }
            },
        }
    }

    stack.pop().unwrap()
}

fn convert_syntax_kind(syntax_kind: sky_sl::syn::cst::SyntaxKind) -> SymbolKind {
    use sky_sl::syn::cst::SyntaxKind;

    match syntax_kind {
        SyntaxKind::Module => SymbolKind::Module,
        SyntaxKind::Struct => SymbolKind::Struct,
        SyntaxKind::Fn => SymbolKind::Function,
        SyntaxKind::ArgumentList => SymbolKind::Unknown,
        SyntaxKind::Argument => SymbolKind::Unknown,
        SyntaxKind::StructKeyword => SymbolKind::Struct,
        SyntaxKind::FnKeyword => SymbolKind::Function,
        SyntaxKind::Identifier => SymbolKind::Unknown,
        SyntaxKind::Whitespace => SymbolKind::Unknown,
        SyntaxKind::Comment => SymbolKind::Unknown,
        SyntaxKind::NumLiteral => SymbolKind::Number,
        SyntaxKind::Semicolon => SymbolKind::Unknown,
        SyntaxKind::Comma => SymbolKind::Unknown,
        SyntaxKind::Dot => SymbolKind::Unknown,
        SyntaxKind::OpenParen => SymbolKind::Unknown,
        SyntaxKind::CloseParen => SymbolKind::Unknown,
        SyntaxKind::OpenBrace => SymbolKind::Unknown,
        SyntaxKind::CloseBrace => SymbolKind::Unknown,
        SyntaxKind::OpenBracket => SymbolKind::Unknown,
        SyntaxKind::CloseBracket => SymbolKind::Unknown,
        SyntaxKind::At => SymbolKind::Unknown,
        SyntaxKind::Pound => SymbolKind::Unknown,
        SyntaxKind::Tilde => SymbolKind::Unknown,
        SyntaxKind::Question => SymbolKind::Unknown,
        SyntaxKind::Colon => SymbolKind::Unknown,
        SyntaxKind::Dollar => SymbolKind::Unknown,
        SyntaxKind::Equals => SymbolKind::Unknown,
        SyntaxKind::Bang => SymbolKind::Unknown,
        SyntaxKind::LessThan => SymbolKind::Unknown,
        SyntaxKind::GreatherThan => SymbolKind::Unknown,
        SyntaxKind::Minus => SymbolKind::Unknown,
        SyntaxKind::And => SymbolKind::Unknown,
        SyntaxKind::VerticalBar => SymbolKind::Unknown,
        SyntaxKind::Plus => SymbolKind::Unknown,
        SyntaxKind::Star => SymbolKind::Unknown,
        SyntaxKind::Slash => SymbolKind::Unknown,
        SyntaxKind::Caret => SymbolKind::Unknown,
        SyntaxKind::Percent => SymbolKind::Unknown,
        SyntaxKind::Error => SymbolKind::Unknown,
    }
}
