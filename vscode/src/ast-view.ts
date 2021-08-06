import * as lc from "vscode-languageclient";
import * as vscode from 'vscode';

// TODO continue once https://github.com/ebkalderon/tower-lsp/issues/256 is released

export interface SyntaxTreeParams {
    textDocument: lc.TextDocumentIdentifier;
    range: lc.Range | null;
}

export const syntaxTree = new lc.RequestType<SyntaxTreeParams, string, void>("sky-sl/syntaxTree");

export class AstProvider implements vscode.TreeDataProvider<SyntaxNode> {
    constructor(private workspaceRoot: string) { }

    getTreeItem(element: SyntaxNode): vscode.TreeItem {
        return element;
    }

    getChildren(element?: SyntaxNode): Thenable<SyntaxNode[]> {
        if (element) {
            return Promise.resolve([]);
        } else {
            return Promise.resolve([new SyntaxNode("Test", vscode.TreeItemCollapsibleState.Collapsed)]);
        }
    }
}

class SyntaxNode extends vscode.TreeItem {
    constructor(
        public readonly label: string,
        public readonly collapsibleState: vscode.TreeItemCollapsibleState
    ) {
        super(label, collapsibleState);
    }
}
