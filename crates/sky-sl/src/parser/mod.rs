use crate::lexer::Token;
use crate::syn::{Parse, cst::*, ast::Root};

mod error;
mod parser;
pub use error::*;
pub use parser::*;

pub fn parse<'a>(token: &'a [Token], input: &'a str) -> Parse<Root> {
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

/// Parses a struct item
///
/// ```
/// struct Example {
///     member: MemberType,
/// }
/// ```
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

        parse_member_list(parser)?;

        parser.bump_if(SyntaxKind::Whitespace)?;
        if !parser.is_at(SyntaxKind::CloseBrace)? {
            parser.emit_error(ErrorKind::NotFound(SyntaxKind::CloseBrace));
        } else {
            parser.bump()?;
        }

        Ok(())
    })
}

/// Parses a list of struct members
/// ```
/// member: MemberType,
/// second_member: SecondMemberType,
/// ```
fn parse_member_list(parser: &mut Parser) -> Result<(), ParseError> {
    parser.node(SyntaxKind::MemberList, |parser| {
        loop {
            parser.bump_if(SyntaxKind::Whitespace)?;

            if !parser.is_at(SyntaxKind::Identifier)? {
                break;
            }

            parse_member(parser)?;

            parser.bump_if(SyntaxKind::Whitespace)?;
            parser.bump_if(SyntaxKind::Comma)?;
        }

        Ok(())
    })
}

/// Parses a single member of a struct
/// ```
/// member: MemberType
/// ```
fn parse_member(parser: &mut Parser) -> Result<(), ParseError> {
    parser.node(SyntaxKind::Member, |parser| {
        parse_identifier(parser)?;

        parser.bump_if(SyntaxKind::Whitespace)?;
        parser.bump_if(SyntaxKind::Colon)?;

        parser.bump_if(SyntaxKind::Whitespace)?;
        // TODO proper type identifier
        parser.bump_if(SyntaxKind::Identifier)?;
        Ok(())
    })
}

/// Parses a function item
/// ```
/// fn example() {
///     
/// }
/// ```
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

/// Parses a list of arguments, excluding parentheses, separated by comma (allows trailing comma)
/// ```
/// one: One, two: Two
/// ```
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

/// Parses a single argument
/// ```
/// one: One
/// ```
fn parse_argument(parser: &mut Parser) -> Result<(), ParseError> {
    parser.node(SyntaxKind::Argument, |parser| {
        parse_identifier(parser)?;

        parser.bump_if(SyntaxKind::Whitespace)?;
        parser.bump_if(SyntaxKind::Colon)?;

        parser.bump_if(SyntaxKind::Whitespace)?;
        // TODO proper type identifier
        parser.bump_if(SyntaxKind::Identifier)?;
        Ok(())
    })
}

/// Parses an identifier
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
