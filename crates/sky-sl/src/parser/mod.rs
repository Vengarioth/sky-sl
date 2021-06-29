use crate::lexer::Token;
use crate::syn::{Parse, cst::*, ast::Root};

mod error;
mod parser;
mod token_set;
pub use error::*;
pub use parser::*;
pub use token_set::*;

pub fn parse<'a>(token: &'a [Token], input: &'a str) -> Parse<Root> {
    let mut parser = Parser::new(token, input);
    let _ = parse_item(&mut parser);
    parser.finish()
}

/// Parses a top-level item, such as structs and functions
fn parse_item(parser: &mut Parser) {
    parser.node(SyntaxKind::Module, |parser| {
        while !parser.eof() {
            match parser.current() {
                // skip whitespaces
                t if t.is_whitespace() => {
                    parser.bump();
                    continue;
                }

                // parse struct
                SyntaxKind::StructKeyword => parse_struct(parser),

                // parse function
                SyntaxKind::FnKeyword => parse_function(parser),

                // otherwise emit an error and recover
                kind => parser.error_and_recover(ErrorKind::Unexpected(kind), &token_set(&[])),
            }
        }
    });
}

/// Parses a struct
///
/// ```ignore
/// struct Example {
///     member: MemberType,
/// }
/// ```
fn parse_struct(parser: &mut Parser) {
    // we already know we want to parse a struct
    parser.node(SyntaxKind::Struct, |parser| {
        // consume the struct keyword token
        parser.bump();

        parser.bump_if(SyntaxKind::Whitespace);

        // parse the struct's name
        parse_identifier(parser);

        parser.bump_if(SyntaxKind::Whitespace);

        if !parser.is_at(SyntaxKind::OpenBrace) {
            parser.error_and_recover(ErrorKind::NotFound(SyntaxKind::OpenBrace), &token_set(&[SyntaxKind::CloseBrace]));
        } else {
            parser.bump();
        }

        parser.bump_if(SyntaxKind::Whitespace);

        // parse all members of the struct
        parse_member_list(parser);

        parser.bump_if(SyntaxKind::Whitespace);

        if !parser.is_at(SyntaxKind::CloseBrace) {
            parser.error_and_recover(ErrorKind::NotFound(SyntaxKind::CloseBrace), &token_set(&[]));
        } else {
            parser.bump();
        }
    });
}

/// Parses zero or more struct members
/// ```ignore
/// member: MemberType,
/// second_member: SecondMemberType,
/// ```
fn parse_member_list(parser: &mut Parser) {
    parser.node(SyntaxKind::MemberList, |parser| {
        loop {
            parser.bump_if(SyntaxKind::Whitespace);

            if !parser.is_at(SyntaxKind::Identifier) {
                break;
            }

            parse_member(parser);

            parser.bump_if(SyntaxKind::Whitespace);

            // TODO the comma token being optional means we parse invalid structs
            parser.bump_if(SyntaxKind::Comma);
        }
    });
}

/// Parses a single member of a struct
/// ```ignore
/// member: MemberType
/// ```
fn parse_member(parser: &mut Parser) {
    parser.node(SyntaxKind::Member, |parser| {
        parse_identifier(parser);

        parser.bump_if(SyntaxKind::Whitespace);

        parser.bump_if(SyntaxKind::Colon);

        parser.bump_if(SyntaxKind::Whitespace);

        // TODO proper type identifier
        parse_type_identifier(parser);
    });
}

/// Parses a function
/// ```ignore
/// fn example() {
///     
/// }
/// ```
fn parse_function(parser: &mut Parser) {
    // we already know we want to parse a function
    parser.node(SyntaxKind::Fn, |parser| {
        // consume the fn keyword
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

        parse_block(parser);

        parser.bump_if(SyntaxKind::Whitespace);
        if !parser.is_at(SyntaxKind::CloseBrace) {
            parser.emit_error(ErrorKind::NotFound(SyntaxKind::CloseBrace));
        } else {
            parser.bump();
        }
    });
}

/// Parses a list of arguments, excluding parentheses, separated by comma (allows trailing comma)
/// ```ignore
/// one: One, two: Two
/// ```
fn parse_arguments(parser: &mut Parser) {
    parser.node(SyntaxKind::ArgumentList, |parser| {
        loop {
            parser.bump_if(SyntaxKind::Whitespace);

            if !parser.is_at(SyntaxKind::Identifier) {
                break;
            }

            parse_argument(parser);

            parser.bump_if(SyntaxKind::Whitespace);
            parser.bump_if(SyntaxKind::Comma);
        }
    });
}

/// Parses a single argument
/// ```ignore
/// one: One
/// ```
fn parse_argument(parser: &mut Parser) {
    parser.node(SyntaxKind::Argument, |parser| {
        parse_identifier(parser);

        parser.bump_if(SyntaxKind::Whitespace);
        parser.bump_if(SyntaxKind::Colon);

        parser.bump_if(SyntaxKind::Whitespace);
        // TODO proper type identifier
        parse_type_identifier(parser);
    });
}

/// Parses an identifier
fn parse_identifier(parser: &mut Parser) {
    parser.bump_if(SyntaxKind::Whitespace);

    parser.node(SyntaxKind::Identifier, |parser| {
        if !parser.is_at(SyntaxKind::Identifier) {
            parser.emit_error(ErrorKind::NotFound(SyntaxKind::Identifier));
        } else {
            parser.bump();
        }
    });
}

fn parse_type_identifier(parser: &mut Parser) {
    // TODO
    parser.bump_if(SyntaxKind::Whitespace);

    parser.node(SyntaxKind::TypeIdentifier, |parser| {
        if !parser.is_at(SyntaxKind::Identifier) {
            parser.emit_error(ErrorKind::NotFound(SyntaxKind::Identifier));
        } else {
            parser.bump();
        }
    });
}

/// Parses a block
fn parse_block(parser: &mut Parser) {
    parser.node(SyntaxKind::Block, |parser| {
        while !parser.is_at(SyntaxKind::CloseBrace) {
            if parser.eof() {
                return parser.error_and_recover(ErrorKind::Unexpected(parser.current()), &token_set(&[]));
            }

            parser.bump_if(SyntaxKind::Whitespace);

            parse_statement(parser);

            parser.bump_if(SyntaxKind::Whitespace);
        }
    });
}

/// Parses a statement
fn parse_statement(parser: &mut Parser) {
    // TODO
    parser.node(SyntaxKind::LetStatement, |parser| {

        if parser.is_at(SyntaxKind::LetKeyword) {
            parse_let_statement(parser);
        } else {
            parse_expression_statement(parser);
        }
    });
}

fn parse_let_statement(parser: &mut Parser) {
    // assume we are at the `let` keyword
    if !parser.is_at(SyntaxKind::LetKeyword) {
        // error
        return parser.error_and_recover(ErrorKind::NotFound(SyntaxKind::LetKeyword), &token_set(&[SyntaxKind::Semicolon]));
    }
    parser.bump();

    parser.bump_if(SyntaxKind::Whitespace);

    // parse the identifier
    parse_identifier(parser);

    
    parser.bump_if(SyntaxKind::Whitespace);

    if parser.is_at(SyntaxKind::Colon) {
        // TODO parse type annotation
    }

    // parse the `=`
    if !parser.is_at(SyntaxKind::Equals) {
        // error
        return parser.error_and_recover(ErrorKind::NotFound(SyntaxKind::Equals), &token_set(&[SyntaxKind::Semicolon]));
    }
    parser.bump();

    parser.bump_if(SyntaxKind::Whitespace);

    parse_expression(parser);

    parser.bump_if(SyntaxKind::Whitespace);

    if !parser.is_at(SyntaxKind::Semicolon) {
        return parser.error_and_recover(ErrorKind::NotFound(SyntaxKind::Semicolon), &token_set(&[]));
    }
    parser.bump();
}

fn parse_expression_statement(parser: &mut Parser) {
    parser.bump_if(SyntaxKind::Whitespace);

    parse_expression(parser);

    parser.bump_if(SyntaxKind::Whitespace);

    if !parser.is_at(SyntaxKind::Semicolon) {
        return parser.error_and_recover(ErrorKind::NotFound(SyntaxKind::Semicolon), &token_set(&[]));
    }
    parser.bump();
}

/// Entry point to parsing expressions
fn parse_expression(parser: &mut Parser) {
    parse_binary_expression(parser);
}

fn parse_binary_expression(parser: &mut Parser) {
    let checkpoint = parser.checkpoint();
    parse_primary_expression(parser);
        
    while let Some(operator) = parse_operator(parser) {
        parser.begin_node_at(checkpoint, SyntaxKind::BinaryExpression);

        parse_expression(parser);
        parser.end_node();
    }
}

fn parse_primary_expression(parser: &mut Parser) {
    let current = parser.ws().current();
    match current {
        SyntaxKind::Minus | SyntaxKind::Bang => {
            // prefix expression (negate)
            todo!();
        },
        _ => {
            parse_atom_expression(parser);
            // parse_postfix_expression(parser)?;
        },
    }
}

fn parse_postfix_expression(parser: &mut Parser) {
    // TODO call
    // TODO dot
    // TODO field
    unimplemented!()
}

fn parse_atom_expression(parser: &mut Parser) {

    let current = parser.ws().current();

    if current == SyntaxKind::OpenParen {
        return parser.node(SyntaxKind::GroupExpression, |parser| {
            parser.bump();
    
            parse_expression(parser);

            if !parser.ws().is_at(SyntaxKind::CloseParen) {
                return parser.error_and_recover(ErrorKind::NotFound(SyntaxKind::CloseParen), &token_set(&[SyntaxKind::Semicolon]));
            }
            parser.bump();
        });
    }

    if let Some(_path) = parse_path_expression(parser) {
        return;
    }

    if let Some(_literal) = parse_literal_expression(parser) {
        return;
    }

    return parser.error_and_recover(ErrorKind::Unexpected(parser.current()), &token_set(&[SyntaxKind::Semicolon]));
}

fn parse_operator(parser: &mut Parser) -> Option<Operator> {
    let current = parser.ws().current();
    let next = parser.next();

    match (current, next) {
        (SyntaxKind::Equals, Some(SyntaxKind::Equals)) => {},
        _ => {},
    }

    if let Some(operator) = current.operator() {
        parser.node(SyntaxKind::Operator, |parser| parser.bump());
        return Some(operator);
    }

    None
}

fn parse_path_expression(parser: &mut Parser) -> Option<()> {
    // TODO proper parsing of path expr
    if parser.ws().is_at(SyntaxKind::Identifier) {
        parser.node(SyntaxKind::PathExpression, |parser| parser.bump());
        Some(())
    } else {
        None
    }
}

fn parse_literal_expression(parser: &mut Parser) -> Option<()> {
    if parser.ws().is_at(SyntaxKind::NumLiteral) {
        parser.node(SyntaxKind::LiteralExpression, |parser| parser.bump());
        Some(())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;

    // A list of complete expressions, to test that the parser is correct
    const VALID_EXPRESSIONS: &[&str] = &[
        "1",
        "(1)",
        "1 + 1",
        "1 + 2 + 3",
        "1 + (2 + 3)",
        "(1 + 2 + 3)",
        "(1 + 2) + 3",
        "a",
        "(a)",
        "a + b",
        "a + (b + c)",
        "(a + b + c)",
        "(a + b) + c",
        "((a + b) + (c))",
        "((a + b)) + c",
        "(((a + b))) + c",
    ];

    // A list of incomplete expressions, to test that the parser terminates
    const INCOMPLETE_EXPRESSIONS: &[&str] = &[
        "",
        "1)",
        "(",
        "(1",
        "(1 + 2",
    ];

    #[test]
    fn it_works() {
        for expr in VALID_EXPRESSIONS.iter().skip(0) {
            let input = format!("fn foo() {{ let a = {}; }}", expr);
            let token = tokenize(&input);
            let parsed = parse(&token, &input);
            assert!(parsed.errors().len() == 0);
        }

        for expr in INCOMPLETE_EXPRESSIONS.iter() {
            let input = format!("fn foo() {{ let a = {}", expr);
            dbg!(&input);
            let token = tokenize(&input);
            let parsed = parse(&token, &input);
            assert!(parsed.errors().len() != 0);
        }
    }
}
