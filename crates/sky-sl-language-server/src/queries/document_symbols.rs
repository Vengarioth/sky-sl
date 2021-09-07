use sky_sl::syn::{ast::*, cst::LineIndex};
use tower_lsp::lsp_types::*;

pub fn document_symbols(ast: Root, line_index: LineIndex) -> DocumentSymbolResponse {
    let mut symbols = Vec::new();

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
            let mut children = Vec::new();
            if let Some(member_list) = struct_definition.member_list() {
                for member in member_list.member() {
                    if let Some(identifier) = member.identifier() {
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
                            kind: SymbolKind::Property,
                            tags: None,
                            range,
                            selection_range,
                            children: None,
                            deprecated: None,
                        };
                        children.push(symbol);
                    }
                }
            }

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
                children: Some(children),
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

        if let Some(signature) = fn_definition.signature() {
            if let Some(identifier) = signature.identifier() {
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
    }

    for module in ast.modules() {
        let syntax = module.syntax();
        let range = syntax.text_range();
        let start = line_index.find_position(range.start());
        let end = line_index.find_position(range.end());
        let range = Range::new(
            Position::new(start.line, start.column),
            Position::new(end.line, end.column),
        );

        if let Some(identifier) = module.identifier() {
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
                kind: SymbolKind::Module,
                tags: None,
                range,
                selection_range,
                children: None,
                deprecated: None,
            };
            symbols.push(symbol);
        }
    }

    DocumentSymbolResponse::Nested(symbols)
}
