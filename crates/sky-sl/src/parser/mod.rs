use crate::lexer::Token;
use crate::syn::cst::*;

mod parser;
pub use parser::*;

pub fn parse(token: &[Token]) -> GreenNode {
    let mut parser = Parser::new(token);
    parse_item(&mut parser);
    parser.process()
}

fn parse_item(parser: &mut Parser) {
    // consume token until we can construct an item
    while !parser.eof() {
        match parser.current() {
            // skip whitespaces
            t if t.is_whitespace() => {
                parser.bump();
                continue;
            },

            // parse struct
            SyntaxKind::StructKeyword => parse_struct(parser),
            
            // parse function
            SyntaxKind::FnKeyword => parse_function(parser),
            
            // otherwise emit an error, consume and then continue
            _ => {
                parser.emit_error(); // Unexpected..
                parser.bump();
            }
        }
    }
}

fn parse_struct(parser: &mut Parser) {
    let marker = parser.begin_node();
    parser.bump();

    parse_identifier(parser);

    marker.complete(parser, SyntaxKind::Struct);
}

fn parse_function(parser: &mut Parser) {
    let marker = parser.begin_node();
    parser.bump();
    
    parse_identifier(parser);

    parser.bump_if(SyntaxKind::Whitespace);
    if !parser.is_at(SyntaxKind::OpenParen) {
        parser.emit_error(); // expected Identifier
    } else {
        parser.bump();
    }

    parser.bump_if(SyntaxKind::Whitespace);
    if !parser.is_at(SyntaxKind::CloseParen) {
        parser.emit_error(); // expected Identifier
    } else {
        parser.bump();
    }

    marker.complete(parser, SyntaxKind::Fn);
}

fn parse_identifier(parser: &mut Parser) {
    parser.bump_if(SyntaxKind::Whitespace);

    if !parser.is_at(SyntaxKind::Identifier) {
        parser.emit_error(); // expected Identifier
    } else {
        parser.bump();
    }
}


#[cfg(test)]
mod tests {
    use crate::lexer::tokenize;
    use super::*;

    #[test]
    fn it_works() {
        let token = tokenize("fn foo() { }");
        dbg!(parse(&token));
    }
}
