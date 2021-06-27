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

        parse_block(parser)?;

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

/// Parses a block
fn parse_block(parser: &mut Parser) -> Result<(), ParseError> {
    parser.node(SyntaxKind::Block, |parser| {
        while !parser.is_at(SyntaxKind::CloseBrace)? {
            parser.bump_if(SyntaxKind::Whitespace)?;

            parse_statement(parser)?;

            parser.bump_if(SyntaxKind::Whitespace)?;
        }
        Ok(())
    })
}

/// Parses a statement
fn parse_statement(parser: &mut Parser) -> Result<(), ParseError> {
    parser.node(SyntaxKind::Statement, |parser| {

        if parser.is_at(SyntaxKind::LetKeyword)? {
            parse_let_statement(parser)?;
        } else {
            parse_expression_statement(parser)?;
        }

        Ok(())
    })
}

fn parse_let_statement(parser: &mut Parser) -> Result<(), ParseError> {
    // assume we are at the `let` keyword
    if !parser.is_at(SyntaxKind::LetKeyword)? {
        // error
        return Ok(());
    }
    parser.bump()?;

    parser.bump_if(SyntaxKind::Whitespace)?;

    // parse the identifier
    parse_identifier(parser)?;

    
    parser.bump_if(SyntaxKind::Whitespace)?;

    // TODO parse type annotation
    if parser.is_at(SyntaxKind::Colon)? {
        parser.bump_if(SyntaxKind::Whitespace)?;
    }

    // parse the `=`
    if !parser.is_at(SyntaxKind::Equals)? {
        // error
        return Ok(());
    }
    parser.bump()?;

    parser.bump_if(SyntaxKind::Whitespace)?;

    parse_expression(parser)?;

    parser.bump_if(SyntaxKind::Whitespace)?;

    if !parser.is_at(SyntaxKind::Semicolon)? {
        // error
        return Ok(());
    }
    parser.bump()?;

    // identifier
    // =
    // expression
    // ;
    Ok(())
}

fn parse_expression_statement(parser: &mut Parser) -> Result<(), ParseError> {
    // expression
    // ;
    Ok(())
}

fn parse_expression(parser: &mut Parser) -> Result<(), ParseError> {
    println!("parse_expression");
    parser.node(SyntaxKind::Expression, |parser| {
        parse_binary_expression(parser)
    })
}

fn parse_binary_expression(parser: &mut Parser) -> Result<(), ParseError> {
    let checkpoint = parser.checkpoint();
    parse_primary_expression(parser)?;
        
    while let Some(operator) = parse_operator(parser)? {
        parser.begin_node_at(checkpoint, SyntaxKind::BinaryExpression);
        dbg!(operator);

        parse_expression(parser)?;
        parser.end_node();
    }

    Ok(())
}

fn parse_primary_expression(parser: &mut Parser) -> Result<(), ParseError> {
    println!("parse_primary_expression");
    let current = parser.ws().current()?;
    match current {
        SyntaxKind::Minus | SyntaxKind::Bang => {
            // prefix expression (negate)
            todo!();
        },
        _ => {
            parse_atom_expression(parser)?;
            // parse_postfix_expression(parser)?;
        },
    }
    Ok(())
}

fn parse_postfix_expression(parser: &mut Parser) -> Result<(), ParseError> {
    println!("parse_postfix_expression");
    loop {
        // TODO call
        // TODO dot
        // TODO field
        break;
    }
    Ok(())
}

fn parse_atom_expression(parser: &mut Parser) -> Result<(), ParseError> {
    println!("parse_atom_expression");
    if let Some(literal) = parse_literal_expression(parser)? {
        dbg!(literal);
        return Ok(());
    }

    // TODO parens etc

    Ok(())
}

fn parse_operator(parser: &mut Parser) -> Result<Option<Operator>, ParseError> {
    println!("parse_operator");
    let current = parser.ws().current()?;

    if let Some(operator) = current.operator() {
        parser.node(SyntaxKind::Plus, |parser| parser.bump())?;
        return Ok(Some(operator));
    }

    Ok(None)
}

fn parse_literal_expression(parser: &mut Parser) -> Result<Option<()>, ParseError> {
    println!("parse_literal_expression");
    if parser.ws().is_at(SyntaxKind::NumLiteral)? {
        parser.node(SyntaxKind::Expression, |parser| parser.bump())?;
        Ok(Some(()))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;

    #[test]
    fn it_works() {
        let input = "fn foo() { let a = 1 + 1; }";
        let token = tokenize(input);
        dbg!(parse(&token, input).tree());
        panic!();
    }
}
