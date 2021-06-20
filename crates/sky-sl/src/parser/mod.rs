use crate::lexer::Token;
use crate::syn::cst::*;

mod error;
mod parser;
pub use error::*;
pub use parser::*;

pub fn parse<'a>(token: &'a [Token], input: &'a str) -> ParseResult {
    let mut parser = Parser::new(token, input);
    let _ = parse_item(&mut parser);
    parser.finish()
}

fn parse_item(parser: &mut Parser) -> Result<(), ParseError> {
    parser.node(SyntaxKind::Module, |parser| {
        while !parser.eof() {
            match parser.current()? {
                // skip whitespaces
                t if t.is_whitespace() => {
                    parser.bump()?;
                    continue;
                }

                // parse struct
                SyntaxKind::StructKeyword => parse_struct(parser)?,

                // parse function
                SyntaxKind::FnKeyword => parse_function(parser)?,

                // otherwise emit an error, consume and then continue
                kind => {
                    parser.emit_error(ErrorKind::Unexpected(kind));
                    parser.bump()?;
                }
            }
        }

        Ok(())
    })
}

fn parse_struct(parser: &mut Parser) -> Result<(), ParseError> {
    parser.node(SyntaxKind::Struct, |parser| {
        parser.bump()?;

        parse_identifier(parser)?;

        parser.bump_if(SyntaxKind::Whitespace)?;
        if !parser.is_at(SyntaxKind::OpenBrace)? {
            parser.emit_error(ErrorKind::NotFound(SyntaxKind::OpenBrace));
            parser.recover(&[SyntaxKind::CloseBrace]);
        } else {
            parser.bump()?;
        }

        // TODO parse member

        parser.bump_if(SyntaxKind::Whitespace)?;
        if !parser.is_at(SyntaxKind::CloseBrace)? {
            parser.emit_error(ErrorKind::NotFound(SyntaxKind::CloseBrace));
        } else {
            parser.bump()?;
        }

        Ok(())
    })
}

fn parse_function(parser: &mut Parser) -> Result<(), ParseError> {
    parser.node(SyntaxKind::Fn, |parser| {
        parser.bump()?;

        parse_identifier(parser)?;

        parser.bump_if(SyntaxKind::Whitespace)?;
        if !parser.is_at(SyntaxKind::OpenParen)? {
            parser.emit_error(ErrorKind::NotFound(SyntaxKind::OpenParen));
        } else {
            parser.bump()?;
        }

        parse_arguments(parser)?;

        parser.bump_if(SyntaxKind::Whitespace)?;
        if !parser.is_at(SyntaxKind::CloseParen)? {
            parser.emit_error(ErrorKind::NotFound(SyntaxKind::CloseParen));
        } else {
            parser.bump()?;
        }

        parser.bump_if(SyntaxKind::Whitespace)?;
        if !parser.is_at(SyntaxKind::OpenBrace)? {
            parser.emit_error(ErrorKind::NotFound(SyntaxKind::OpenBrace));
        } else {
            parser.bump()?;
        }

        // TODO parse block

        parser.bump_if(SyntaxKind::Whitespace)?;
        if !parser.is_at(SyntaxKind::CloseBrace)? {
            parser.emit_error(ErrorKind::NotFound(SyntaxKind::CloseBrace));
        } else {
            parser.bump()?;
        }

        Ok(())
    })
}

fn parse_arguments(parser: &mut Parser) -> Result<(), ParseError> {
    parser.node(SyntaxKind::ArgumentList, |parser| {
        loop {
            parser.bump_if(SyntaxKind::Whitespace)?;

            if !parser.is_at(SyntaxKind::Identifier)? {
                break;
            }

            parse_argument(parser)?;

            parser.bump_if(SyntaxKind::Whitespace)?;
            parser.bump_if(SyntaxKind::Comma)?;
        }

        Ok(())
    })
}

fn parse_argument(parser: &mut Parser) -> Result<(), ParseError> {
    parser.node(SyntaxKind::Argument, |parser| {
        parser.bump()?;

        parser.bump_if(SyntaxKind::Whitespace)?;
        parser.bump_if(SyntaxKind::Colon)?;

        parser.bump_if(SyntaxKind::Whitespace)?;
        // TODO proper type identifier
        parser.bump_if(SyntaxKind::Identifier)?;
        Ok(())
    })
}

fn parse_identifier(parser: &mut Parser) -> Result<(), ParseError> {
    parser.bump_if(SyntaxKind::Whitespace)?;

    parser.node(SyntaxKind::Identifier, |parser| {
        if !parser.is_at(SyntaxKind::Identifier)? {
            parser.emit_error(ErrorKind::NotFound(SyntaxKind::Identifier));
        } else {
            parser.bump()?;
        }

        Ok(())
    })
}

fn parse_block(parser: &mut Parser) -> Result<(), ParseError> {
    parser.bump_if(SyntaxKind::Whitespace)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;

    #[test]
    fn it_works() {
        let input = "fn foo() { }";
        let token = tokenize(input);
        dbg!(parse(&token, input));
    }
}
