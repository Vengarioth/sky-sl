import * as path from 'path';
import { workspace, ExtensionContext, window } from 'vscode';
import { LanguageClient, LanguageClientOptions, ServerOptions, TransportKind } from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: ExtensionContext) {
    let serverCommand = context.asAbsolutePath(
        path.join('../', 'target', 'debug', 'sky-sl-language-server.exe')
    );
    
    const serverOptions: ServerOptions = {
        run: { command: serverCommand, transport: TransportKind.stdio },
        debug: { command: serverCommand, transport: TransportKind.stdio },
    };

    const traceOutputChannel = window.createOutputChannel(
        "sky-sl language server trace"
    );

    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: 'file', language: 'skysl' }],
        initializationOptions: workspace.getConfiguration('skysl'),
        // synchronize: {
        //     fileEvents: workspace.createFileSystemWatcher('**/*.skysl'),
        // },
        diagnosticCollectionName: "skysl",
        traceOutputChannel,
    };

    client = new LanguageClient('sky-sl', 'skysl language server', serverOptions, clientOptions);

    client.start();
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    
    return client.stop();
}
