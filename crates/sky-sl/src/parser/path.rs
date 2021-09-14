use super::{token_set, ErrorKind, Parser};
use crate::syn::cst::SyntaxKind;

/// Parses a path e.g. `foo::bar`
pub fn parse_path(parser: &mut Parser) {
    parser.begin_node(SyntaxKind::Path);

    // parse the first identifier
    if !parser.is_at(SyntaxKind::Identifier) {
        parser.emit_error(ErrorKind::NotFound(SyntaxKind::Identifier));
    } else {
        parser.bump();
    }

    // check for a `::`
    if parser.is_at(SyntaxKind::Colon) {
        parser.bump();
        if !parser.is_at(SyntaxKind::Colon) {
            parser.error_and_recover(
                ErrorKind::NotFound(SyntaxKind::Colon),
                &token_set(&[
                    SyntaxKind::Identifier,
                    SyntaxKind::OpenBrace,
                    SyntaxKind::Comma,
                    SyntaxKind::CloseBrace,
                    SyntaxKind::Semicolon,
                ]),
            )
        }

        if parser.is_at(SyntaxKind::OpenBrace) {
            parser.bump();

            parser.ws();

            if !parser.is_at(SyntaxKind::CloseBrace) {
                parser.error_and_recover(
                    ErrorKind::NotFound(SyntaxKind::CloseBrace),
                    &token_set(&[
                        SyntaxKind::Identifier,
                        SyntaxKind::OpenBrace,
                        SyntaxKind::Comma,
                        SyntaxKind::Colon,
                        SyntaxKind::Semicolon,
                    ]),
                )
            } else {
                parser.bump();
            }
        } else {
            parse_path(parser);
        }
    }

    parser.end_node();
}

fn parse_use_tree() {

}

fn parse_use_group() {

}

fn parse_use_identifier() {
    
}

#[cfg(test)]
mod tests {
    use crate::{lexer, parser};

    #[test]
    fn it_works() {
        let input = "use foo::bar;";
        let tokens = lexer::tokenize(input);
        let root = parser::parse(&tokens, input);
        dbg!(root.tree());
    }
}
