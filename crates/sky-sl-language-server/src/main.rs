use serde::{Deserialize, Serialize};
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LspService, Server};
use std::sync::Mutex;

mod semantics;
mod workspaces;
mod vfs;
mod handlers;
mod lsp;

use workspaces::Workspaces;

#[derive(Debug, Deserialize, Serialize)]
struct SyntaxTreeParams {
    text_ocument: TextDocumentIdentifier,
    range: Option<Range>,
}

struct Backend {
    client: Client,
    workspaces: Mutex<Workspaces>,
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, messages) = LspService::new(|client| Backend {
        client,
        workspaces: Mutex::new(Workspaces::new()),
    });

    Server::new(stdin, stdout)
        .interleave(messages)
        .serve(service)
        .await;
}
