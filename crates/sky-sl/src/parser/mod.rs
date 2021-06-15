use crate::lexer::Token;
use crate::syn::cst::*;

mod error;
mod parser;
pub use error::*;
pub use parser::*;

pub fn parse<'a>(token: &'a [Token], input: &'a str) -> ParseResult {
    let mut parser = Parser::new(token, input);
    parse_item(&mut parser);
    parser.finish()
}

fn parse_item(parser: &mut Parser) {
    parser.begin_node(SyntaxKind::Module);
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
            kind => {
                parser.emit_error(ErrorKind::Unexpected(kind));
                parser.bump();
            }
        }
    }
    parser.end_node();
}

fn parse_struct(parser: &mut Parser) {
    parser.begin_node(SyntaxKind::Struct);
    parser.bump();

    parse_identifier(parser);

    parser.bump_if(SyntaxKind::Whitespace);
    if !parser.is_at(SyntaxKind::OpenBrace) {
        parser.emit_error(ErrorKind::NotFound(SyntaxKind::OpenBrace));
        parser.recover(&[SyntaxKind::CloseBrace]);
    } else {
        parser.bump();
    }

    // TODO parse member

    parser.bump_if(SyntaxKind::Whitespace);
    if !parser.is_at(SyntaxKind::CloseBrace) {
        parser.emit_error(ErrorKind::NotFound(SyntaxKind::CloseBrace));
    } else {
        parser.bump();
    }

    parser.end_node();
}

fn parse_function(parser: &mut Parser) {
    parser.begin_node(SyntaxKind::Fn);
    parser.bump();
    
    parse_identifier(parser);

    parser.bump_if(SyntaxKind::Whitespace);
    if !parser.is_at(SyntaxKind::OpenParen) {
        parser.emit_error(ErrorKind::NotFound(SyntaxKind::OpenParen));
    } else {
        parser.bump();
    }

    parse_arguments(parser);

    parser.bump_if(SyntaxKind::Whitespace);
    if !parser.is_at(SyntaxKind::CloseParen) {
        parser.emit_error(ErrorKind::NotFound(SyntaxKind::CloseParen));
    } else {
        parser.bump();
    }

    parser.bump_if(SyntaxKind::Whitespace);
    if !parser.is_at(SyntaxKind::OpenBrace) {
        parser.emit_error(ErrorKind::NotFound(SyntaxKind::OpenBrace));
    } else {
        parser.bump();
    }

    // TODO parse block

    parser.bump_if(SyntaxKind::Whitespace);
    if !parser.is_at(SyntaxKind::CloseBrace) {
        parser.emit_error(ErrorKind::NotFound(SyntaxKind::CloseBrace));
    } else {
        parser.bump();
    }

    parser.end_node();
}

fn parse_arguments(parser: &mut Parser) {
    
    parser.begin_node(SyntaxKind::ArgumentList);
    
    loop {
        parser.bump_if(SyntaxKind::Whitespace);

        if !parser.is_at(SyntaxKind::Identifier) {
            break;
        }

        parser.begin_node(SyntaxKind::Argument);
        parser.bump();

        parser.bump_if(SyntaxKind::Whitespace);
        parser.bump_if(SyntaxKind::Colon);
        
        parser.bump_if(SyntaxKind::Whitespace);
        // TODO proper type identifier
        parser.bump_if(SyntaxKind::Identifier);
        
        parser.end_node();

        parser.bump_if(SyntaxKind::Whitespace);
        parser.bump_if(SyntaxKind::Comma);
    }


    parser.end_node();
}

fn parse_identifier(parser: &mut Parser) {
    parser.bump_if(SyntaxKind::Whitespace);
    
    parser.begin_node(SyntaxKind::Identifier);

    if !parser.is_at(SyntaxKind::Identifier) {
        parser.emit_error(ErrorKind::NotFound(SyntaxKind::Identifier));
    } else {
        parser.bump();
    }

    parser.end_node();
}

fn parse_block(parser: &mut Parser) {
    parser.bump_if(SyntaxKind::Whitespace);
}

#[cfg(test)]
mod tests {
    use crate::lexer::tokenize;
    use super::*;

    #[test]
    fn it_works() {
        let input = "fn foo() { }";
        let token = tokenize(input);
        dbg!(parse(&token, input));
    }
}
