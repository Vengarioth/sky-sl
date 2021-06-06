import * as path from 'path';
import { workspace, ExtensionContext } from 'vscode';
import { LanguageClient, LanguageClientOptions, ServerOptions, TransportKind } from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: ExtensionContext) {
    let serverCommand = context.asAbsolutePath(
        path.join('../', 'target', 'debug', 'sky-sl-language-server.exe')
    );
    
    const serverOptions: ServerOptions = {
        run: { command: serverCommand, },
        debug: { command: serverCommand, },
    };

    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: 'file', language: 'sky-sl' }],
        synchronize: {fileEvents: workspace.createFileSystemWatcher('**/.skysl')}
    };

    client = new LanguageClient('sky-sl', 'Sky-SL language server', serverOptions, clientOptions);

    client.start();
}

export function deactivate(): Thenable<void> | undefined {
    return undefined;
}
