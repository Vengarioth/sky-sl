use crate::{lexer::Token, syn::cst::UnaryOperator};

mod diagnostic;
mod parser;

pub use diagnostic::*;
use parser::*;
pub use parser::ParseResult;

use super::cst::{BinaryOperator, Checkpoint, SyntaxKind};

macro_rules! t {
    [.] => {$crate::syn::cst::SyntaxKind::Dot};
    [:] => {$crate::syn::cst::SyntaxKind::Colon};
    [;] => {$crate::syn::cst::SyntaxKind::Semicolon};
    [,] => {$crate::syn::cst::SyntaxKind::Comma};
    [+] => {$crate::syn::cst::SyntaxKind::Plus};
    [-] => {$crate::syn::cst::SyntaxKind::Minus};
    [*] => {$crate::syn::cst::SyntaxKind::Star};
    [/] => {$crate::syn::cst::SyntaxKind::Slash};
    [=] => {$crate::syn::cst::SyntaxKind::Equals};
    [%] => {$crate::syn::cst::SyntaxKind::Percent};
    [&] => {$crate::syn::cst::SyntaxKind::And};
    [|] => {$crate::syn::cst::SyntaxKind::VerticalBar};
    [!] => {$crate::syn::cst::SyntaxKind::Bang};
    [?] => {$crate::syn::cst::SyntaxKind::Question};
    [^] => {$crate::syn::cst::SyntaxKind::Caret};
    [~] => {$crate::syn::cst::SyntaxKind::Tilde};
    ['{'] => {$crate::syn::cst::SyntaxKind::OpenBrace};
    ['}'] => {$crate::syn::cst::SyntaxKind::CloseBrace};
    ['('] => {$crate::syn::cst::SyntaxKind::OpenParen};
    [')'] => {$crate::syn::cst::SyntaxKind::CloseParen};
    [<] => {$crate::syn::cst::SyntaxKind::LessThan};
    [>] => {$crate::syn::cst::SyntaxKind::GreatherThan};
    [mod] => {$crate::syn::cst::SyntaxKind::ModKeyword};
    [use] => {$crate::syn::cst::SyntaxKind::UseKeyword};
    [fn] => {$crate::syn::cst::SyntaxKind::FnKeyword};
    [struct] => {$crate::syn::cst::SyntaxKind::StructKeyword};
    [let] => {$crate::syn::cst::SyntaxKind::LetKeyword};
    [if] => {$crate::syn::cst::SyntaxKind::IfKeyword};
    [else] => {$crate::syn::cst::SyntaxKind::ElseKeyword};
    [loop] => {$crate::syn::cst::SyntaxKind::LoopKeyword};
    [while] => {$crate::syn::cst::SyntaxKind::WhileKeyword};
    [for] => {$crate::syn::cst::SyntaxKind::ForKeyword};
    [true] => {$crate::syn::cst::SyntaxKind::TrueKeyword};
    [false] => {$crate::syn::cst::SyntaxKind::FalseKeyword};
    [package] => {$crate::syn::cst::SyntaxKind::PackageKeyword};
    [ident] => {$crate::syn::cst::SyntaxKind::Identifier};
    [int] => {$crate::syn::cst::SyntaxKind::IntLiteral};
    [float] => {$crate::syn::cst::SyntaxKind::FloatLiteral};
    [bool] => {$crate::syn::cst::SyntaxKind::BoolLiteral};
}

pub fn parse<'a>(token: &'a [Token], input: &'a str) -> ParseResult {
    let mut parser = Parser::new(token, input);
    parse_module(&mut parser);
    parser.finish()
}

/// parses a module
fn parse_module(parser: &mut Parser) {
    parser.node(SyntaxKind::Module, |parser| {
        while !parser.eof() {
            match parser.current() {
                t if t.is_whitespace() => {
                    parser.consume(SyntaxKind::Whitespace);
                }
                SyntaxKind::ModKeyword => parse_module_declaration(parser),
                SyntaxKind::UseKeyword => parse_use_declaration(parser),
                SyntaxKind::StructKeyword => parse_struct_declaration(parser),
                SyntaxKind::FnKeyword => parse_function_declaration(parser),
                _ => unimplemented!(),
            }
        }
    });
}

/// parses a module declaration, e.g. `mod module_name;`
fn parse_module_declaration(parser: &mut Parser) {
    parser.node(SyntaxKind::ModuleDeclaration, |parser| {
        parser.consume(t![mod]);
        parser.ws1();

        parser.expect(t![ident], &[t![;], t![mod], t![use], t![fn], t![struct]]);

        parser.ws0();

        parser.expect(t![;], &[t![mod], t![use], t![fn], t![struct]]);
    });
}

/// parses a use declaration, e.g. `use module_b::{module_c::module_d, module_e}`
fn parse_use_declaration(parser: &mut Parser) {
    parser.node(SyntaxKind::UseDeclaration, |parser| {
        parser.consume(t![use]);
        parser.ws1();

        parse_use_tree(parser);

        parser.expect(t![;], &[t![mod], t![use], t![fn], t![struct]]);
    });
}

/// parses a use tree, e.g. `module_b::module_c`
fn parse_use_tree(parser: &mut Parser) {
    parser.begin_node(SyntaxKind::UseTree);
    parse_use_segment(parser);
    parser.ws0();

    if parser.is_at(SyntaxKind::Colon) {
        parser.expect(
            t![:],
            &[
                t![;],
                t!['{'],
                t!['}'],
                t![mod],
                t![use],
                t![fn],
                t![struct],
            ],
        );
        parser.expect(
            t![:],
            &[
                t![;],
                t!['{'],
                t!['}'],
                t![mod],
                t![use],
                t![fn],
                t![struct],
            ],
        );

        parser.ws0();
        if parser.is_at(t![*]) {
            parser.begin_node(SyntaxKind::UseAll);
            parser.consume(t![*]);
            parser.end_node();
        } else if parser.is_at(SyntaxKind::OpenBrace) {
            parse_use_group(parser);
        } else {
            parse_use_tree(parser);
        }
    }

    parser.end_node();
}

/// parses a use group, e.g. `{module_a, module_b::c}`
fn parse_use_group(parser: &mut Parser) {
    parser.begin_node(SyntaxKind::UseGroup);
    parser.consume(t!['{']);
    parser.ws0();

    loop {
        if !parser.is_at(t![ident]) {
            break;
        }

        parse_use_tree(parser);

        parser.ws0();

        if !parser.consume_if(t![,]) {
            break;
        }

        parser.ws0();
    }

    parser.expect(
        t!['}'],
        &[t![:], t![;], t![mod], t![use], t![fn], t![struct]],
    );
    parser.end_node();
}

/// parses a use segment, meaning an identifier or a `package` keyword
fn parse_use_segment(parser: &mut Parser) {
    parser.begin_node(SyntaxKind::UseSegment);
    parser.expect_any(
        &[t![ident], t![package]],
        &[
            t!['{'],
            t!['}'],
            t![,],
            t![:],
            t![;],
            t![mod],
            t![use],
            t![fn],
            t![struct],
        ],
    );
    parser.end_node();
}

/// parses an entire struct declaration, e.g. `struct MyStruct { member: MemberType, }`
fn parse_struct_declaration(parser: &mut Parser) {
    parser.node(SyntaxKind::Struct, |parser| {
        // parse struct keyword
        parser.consume(t![struct]);
        parser.ws0();

        // parse the struct name
        parser.begin_node(SyntaxKind::Name);
        parser.expect(
            t![ident],
            &[t!['{'], t!['}'], t![mod], t![use], t![fn], t![struct]],
        );
        parser.end_node();
        parser.ws0();

        // parse open brace
        parser.expect(
            SyntaxKind::OpenBrace,
            &[SyntaxKind::CloseBrace, t![mod], t![use], t![fn], t![struct]],
        );
        parser.ws0();

        parse_struct_member_list(parser);

        // parse close brace
        parser.expect(
            SyntaxKind::CloseBrace,
            &[t![mod], t![use], t![fn], t![struct]],
        );
    });
}

/// parses struct members repeatedly, e.g. `member_a: MemberTypeA, member_b: MemberTypeB,`
fn parse_struct_member_list(parser: &mut Parser) {
    parser.node(SyntaxKind::MemberList, |parser| {
        loop {
            if !parser.is_at(SyntaxKind::Identifier) {
                break;
            }

            parser.begin_node(SyntaxKind::Member);

            // parse the member name
            parser.begin_node(SyntaxKind::Name);
            parser.expect(
                SyntaxKind::Identifier,
                &[
                    SyntaxKind::CloseBrace,
                    t![:],
                    t![,],
                    t![mod],
                    t![use],
                    t![fn],
                    t![struct],
                ],
            );
            parser.end_node();
            parser.ws0();

            // parse the : before the type
            parser.expect(
                t![:],
                &[
                    SyntaxKind::CloseBrace,
                    t![,],
                    t![mod],
                    t![use],
                    t![fn],
                    t![struct],
                ],
            );
            parser.ws0();

            // parse the type
            parse_item_path(parser);
            parser.ws0();

            parser.end_node();

            // parse the comma
            if !parser.consume_if(t![,]) {
                break;
            }

            parser.ws0();
        }

        parser.ws0();
    });
}

/// parses an item path that targets a singular item, e.g. `foo::bar::Baz`
fn parse_item_path(parser: &mut Parser) {
    parser.begin_node(SyntaxKind::Path);
    parse_item_path_segment(parser);
    parser.end_node();
}

/// recursively parses path segments, e.g. `foo::bar::Baz`
fn parse_item_path_segment(parser: &mut Parser) {
    parser.begin_node(SyntaxKind::PathSegment);

    parser.begin_node(SyntaxKind::Name);
    parser.expect(
        t![ident],
        &[t![:], t![,], t![;], t![mod], t![use], t![fn], t![struct]],
    );
    parser.end_node();
    parser.ws0();

    if parser.consume_if(t![:]) {
        parser.expect(
            t![:],
            &[
                t![ident],
                t![,],
                t![;],
                t![mod],
                t![use],
                t![fn],
                t![struct],
            ],
        );
        parser.ws0();

        parse_item_path_segment(parser);
    }

    parser.end_node();
}

/// parse a function declaration, e.g. `fn my_function() {}`
fn parse_function_declaration(parser: &mut Parser) {
    parser.begin_node(SyntaxKind::Fn);
    // parse fn keyword
    parser.consume(t![fn]);
    parser.ws0();

    parser.begin_node(SyntaxKind::FnSignature);

    // parse the function name
    parser.begin_node(SyntaxKind::Name);
    parser.expect(
        t![ident],
        &[
            t!['('],
            t![')'],
            t![-],
            t![>],
            t!['{'],
            t!['}'],
            t![mod],
            t![use],
            t![fn],
            t![struct],
        ],
    );
    parser.end_node();
    parser.ws0();

    parser.expect(
        SyntaxKind::OpenParen,
        &[
            t![-],
            t![>],
            t![:],
            t!['{'],
            t!['}'],
            t![ident],
            t![mod],
            t![use],
            t![fn],
            t![struct],
        ],
    );
    parser.ws0();

    // parse the function arguments
    parse_argument_list(parser);

    parser.expect(
        SyntaxKind::CloseParen,
        &[
            t![-],
            t![>],
            t!['{'],
            t!['}'],
            t![ident],
            t![mod],
            t![use],
            t![fn],
            t![struct],
        ],
    );
    parser.ws0();

    // parse optional return type
    if parser.is_at(t![-]) {
        parser.begin_node(SyntaxKind::ReturnType);
        parser.consume(t![-]);
        parser.expect(
            t![>],
            &[
                t![ident],
                t![:],
                t!['{'],
                t!['}'],
                t![mod],
                t![use],
                t![fn],
                t![struct],
            ],
        );
        parser.ws0();
        parse_item_path(parser);
        parser.end_node();
        parser.ws0();
    }

    parser.end_node();

    parse_block(parser);

    parser.end_node();
}

/// parses a list of arguments, e.g. `(foo: Bar)`
fn parse_argument_list(parser: &mut Parser) {
    parser.begin_node(SyntaxKind::ArgumentList);

    loop {
        if !parser.is_at(SyntaxKind::Identifier) {
            break;
        }

        parse_argument(parser);
        parser.ws0();

        if parser.is_at(t![,]) {
            parser.consume(t![,]);
            parser.ws0();
        } else {
            break;
        }
    }

    parser.end_node();
}

/// parses a single argument, e.g. `foo: bar::Bar`
fn parse_argument(parser: &mut Parser) {
    parser.begin_node(SyntaxKind::Argument);
    parser.begin_node(SyntaxKind::Name);
    // consume the identifier
    parser.consume(SyntaxKind::Identifier);
    parser.end_node();
    parser.ws0();

    parser.expect(
        t![:],
        &[
            t![:],
            t![,],
            t!['('],
            t![')'],
            t![-],
            t![>],
            t![ident],
            t![mod],
            t![use],
            t![fn],
            t![struct],
        ],
    );
    parser.ws0();

    parse_item_path(parser);
    parser.end_node();
}

fn parse_block(parser: &mut Parser) {
    parser.begin_node(SyntaxKind::Block);
    parser.expect(t!['{'], &[]);
    parser.ws0();

    parse_statements(parser);
    parser.ws0();

    parser.expect(t!['}'], &[t![mod], t![use], t![fn], t![struct]]);
    parser.end_node();
}

fn parse_statements(parser: &mut Parser) {
    loop {
        match parser.current() {
            // parse a let statement
            t![let] => {
                parse_let_statement(parser);
                parser.ws0();
            },
    
            // parse an expression statement
            // if and loop expression start
            t![if] | t![loop] | t![while] | t![for] |
            // group expression start
            SyntaxKind::OpenParen |
            // primary expression start
            t![ident] | t![int] | t![float] | t![bool] | t![true] | t![false] |
            // unary operators
            t![+] | t![-] | t![!] | t![~]  => {
                parse_expression_statement(parser);
                parser.ws0();
            }
    
            _ => {
                // no more statements to parse
                break;
            },
        }
    }
}

fn parse_let_statement(parser: &mut Parser) {
    parser.begin_node(SyntaxKind::LetStatement);
    
    parser.consume(t![let]);
    parser.ws1();

    parser.begin_node(SyntaxKind::Name);
    parser.expect(t![ident], &[t![:], t![=], t![;], t![mod], t![use], t![fn], t![struct]]);
    parser.end_node();
    parser.ws0();

    if parser.consume_if(t![:]) {
        parser.ws0();

        parser.begin_node(SyntaxKind::TypeIdentifier);
        parse_item_path(parser);
        parser.end_node();
        parser.ws0();
    }

    parser.expect(t![=], &[t![;], t![mod], t![use], t![fn], t![struct]]);
    parser.ws0();

    parse_expression(parser);
    parser.ws0();

    parser.expect(t![;], &[t![mod], t![use], t![fn], t![struct]]);
    parser.end_node();
}

fn parse_expression_statement(parser: &mut Parser) -> bool {
    parser.begin_node(SyntaxKind::ExpressionStatement);
    parse_expression(parser);
    parser.ws0();
    let has_semicolon = parser.consume_if(t![;]);
    parser.end_node();
    has_semicolon
}

fn parse_expression(parser: &mut Parser) {
    match parser.current() {
        t![if] => parse_if_expression(parser),
        t![loop] => parse_unbounded_loop_expression(parser),
        t![while] => parse_predicate_loop_expression(parser),
        t![for] => parse_iterator_loop_expression(parser),
        _ => parse_binary_expression(parser, 1),
    }
}

fn parse_if_expression(parser: &mut Parser) {
    parser.consume(t![if]);
    parser.ws0();

    parse_expression(parser);
    parser.ws0();

    parse_block(parser);
    parser.ws0();

    loop {
        if parser.consume_if(t![else]) {
            parser.ws0();

            if parser.consume_if(t![if]) {
                parser.ws0();

                parse_expression(parser);
                parser.ws0();

                parse_block(parser);
                parser.ws0();
            }

        } else {
            break;
        }
    }

    // "else if" expression

    // "else" expression block
}

fn parse_unbounded_loop_expression(_parser: &mut Parser) {
    unimplemented!();
}

fn parse_predicate_loop_expression(_parser: &mut Parser) {
    unimplemented!();
}

fn parse_iterator_loop_expression(_parser: &mut Parser) {
    unimplemented!();
}

fn parse_binary_expression(parser: &mut Parser, min_precedence: u8) {
    let checkpoint = parser.checkpoint();
    parse_atom(parser);

    while let Some(operator) = peek_binary_operator(parser) {
        if operator.precedence() < min_precedence {
            break;
        }

        let next_min_precedence = if operator.associativity().is_left() {
            operator.precedence() + 1
        } else {
            operator.precedence()
        };

        parser.begin_node(SyntaxKind::Operator);
        parse_binary_operator(parser);
        parser.end_node();

        parser.ws0();

        parser.begin_node_at(checkpoint, SyntaxKind::BinaryExpression);
        parse_binary_expression(parser, next_min_precedence);
        parser.end_node();
    }
}

fn parse_atom(parser: &mut Parser) {
    let checkpoint = parser.checkpoint();
    // parse prefix unary operators, e.g. `-1` or `!true`
    if let Some(_unary_operator) = parse_unary_operator(parser) {
        parser.begin_node_at(checkpoint, SyntaxKind::UnaryExpression);
        parser.begin_node_at(checkpoint, SyntaxKind::Operator);
        parser.end_node();
        // recurse here to parse nested unary operators
        parse_atom(parser);
        parser.end_node();
        parser.ws0();
    } else {
        parse_primary_expression(parser);
        parser.ws0();

        // parse postfix unary operators
        loop {
            match parser.current() {
                SyntaxKind::OpenBracket => {
                    // parse indexing operator, e.g. `a[b]`
                    parse_index_operator(parser, checkpoint);
                    parser.ws0();
                },
                SyntaxKind::OpenParen => {
                    // parse call operator, e.g. `a(b)`
                    parse_call_operator(parser, checkpoint);
                    parser.ws0();
                },
                t![.] => {
                    // parse dot operator, e.g. `a.b`
                    parse_dot_operator(parser, checkpoint);
                    parser.ws0();
                },
                _ => break,
            }
        }
    }
}

fn parse_index_operator(parser: &mut Parser, checkpoint: Checkpoint) {
    parser.begin_node_at(checkpoint, SyntaxKind::IndexExpression);
    parser.consume(SyntaxKind::OpenBracket);
    parser.ws0();

    parser.begin_node(SyntaxKind::Indexer);
    parse_expression(parser);
    parser.end_node();
    parser.ws0();

    parser.expect(SyntaxKind::CloseBracket, &[t![;], SyntaxKind::CloseBrace, t![mod], t![use], t![fn], t![struct]]);
    parser.end_node();
}

fn parse_call_operator(parser: &mut Parser, checkpoint: Checkpoint) {
    parser.begin_node_at(checkpoint ,SyntaxKind::CallExpression);
    parser.consume(SyntaxKind::OpenParen);
    parser.ws0();

    parser.begin_node(SyntaxKind::CallArgumentList);
    loop {
        if !is_at_expression_start(parser) {
            break;
        }

        parser.begin_node(SyntaxKind::CallArgument);
        parse_expression(parser);
        parser.end_node();
        parser.ws0();

        if !parser.is_at(t![,]) {
            break;
        }

        parser.consume(t![,]);
        parser.ws0();
    }
    parser.end_node();

    parser.expect(SyntaxKind::CloseParen, &[t![;], SyntaxKind::CloseBrace, t![mod], t![use], t![fn], t![struct]]);
    parser.end_node();
}

fn parse_dot_operator(parser: &mut Parser, checkpoint: Checkpoint) {
    parser.begin_node_at(checkpoint, SyntaxKind::FieldAccessExpression);
    parser.consume(t![.]);
    parser.ws0();

    parser.expect(t![ident], &[SyntaxKind::OpenParen, SyntaxKind::CloseParen, SyntaxKind::CloseBracket, SyntaxKind::CloseBrace, t![;], t![mod], t![use], t![fn], t![struct]]);

    parser.end_node();
}

fn parse_primary_expression(parser: &mut Parser) {
    match parser.current() {
        SyntaxKind::OpenParen => {
            parse_group_expression(parser);
            parser.ws0();
        },
        t![true] | t![false] => {
            parse_bool_literal(parser);
            parser.ws0();
        },
        t![int] => {
            parse_int_literal(parser);
            parser.ws0();
        },
        t![float] => {
            parse_float_literal(parser);
            parser.ws0();
        },
        t![ident] => {
            parse_item_path(parser);
            parser.ws0();
        },
        _ => parser.missing(&[t![true], t![false], t![int], t![float], t![ident]]),
    }
}

fn parse_group_expression(parser: &mut Parser) {
    parser.begin_node(SyntaxKind::GroupExpression);
    parser.consume(SyntaxKind::OpenParen);
    parser.ws0();

    parse_expression(parser);
    parser.ws0();

    parser.expect(SyntaxKind::CloseParen, &[t![;], SyntaxKind::CloseBrace, t![mod], t![use], t![fn], t![struct]]);
    parser.end_node();
}

fn parse_bool_literal(parser: &mut Parser) {
    parser.begin_node(SyntaxKind::LiteralExpression);
    parser.consume_any(&[t![true], t![false]]);
    parser.end_node();
}

fn parse_int_literal(parser: &mut Parser) {
    parser.begin_node(SyntaxKind::LiteralExpression);
    parser.consume(SyntaxKind::IntLiteral);
    parser.end_node();
}

fn parse_float_literal(parser: &mut Parser) {
    parser.begin_node(SyntaxKind::LiteralExpression);
    parser.consume(SyntaxKind::FloatLiteral);
    parser.end_node();
}

fn peek_binary_operator(parser: &mut Parser) -> Option<BinaryOperator> {
    match parser.current() {
        t![<] => {
            match parser.next() {
                Some(t![<]) => {
                    Some(BinaryOperator::LeftShift)
                },
                Some(t![=]) => {
                    Some(BinaryOperator::LessOrEqualThan)
                },
                _ => Some(BinaryOperator::LessThan)
            }
        },
        t![>] => {
            match parser.next() {
                Some(t![>]) => {
                    Some(BinaryOperator::RightShift)
                },
                Some(t![=]) => {
                    Some(BinaryOperator::GreatherOrEqualThan)
                },
                _ => Some(BinaryOperator::GreaterThan)
            }
        },
        t![=] => {
            match parser.next() {
                Some(t![=]) => {
                    Some(BinaryOperator::Equals)
                },
                _ => {
                    Some(BinaryOperator::Equals)
                }
            }
        },
        t![!] => {
            match parser.next() {
                Some(t![=]) => {
                    Some(BinaryOperator::NotEquals)
                },
                _ => {
                    Some(BinaryOperator::NotEquals)
                }
            }
        },
        t![+] => {
            Some(BinaryOperator::Add)
        },
        t![-] => {
            Some(BinaryOperator::Subtract)
        },
        t![*] => {
            match parser.next() {
                Some(t![*]) => {
                    Some(BinaryOperator::Exponent)
                },
                _ => {
                    Some(BinaryOperator::Multiply)
                }
            }
        },
        t![/] => {
            Some(BinaryOperator::Divide)
        },
        t![%] => {
            Some(BinaryOperator::Remainder)
        },
        t![&] => {
            Some(BinaryOperator::BitwiseAnd)
        },
        t![|] => {
            Some(BinaryOperator::BitwiseOr)
        },
        t![^] => {
            Some(BinaryOperator::BitwiseXOr)
        },
        _ => None,
    }
}

fn parse_binary_operator(parser: &mut Parser) -> Option<BinaryOperator> {
    match parser.current() {
        t![<] => {
            parser.consume(t![<]);
            match parser.current() {
                t![<] => {
                    parser.consume(t![<]);
                    Some(BinaryOperator::LeftShift)
                },
                t![=] => {
                    parser.consume(t![=]);
                    Some(BinaryOperator::LessOrEqualThan)
                },
                _ => Some(BinaryOperator::LessThan)
            }
        },
        t![>] => {
            parser.consume(t![>]);
            match parser.current() {
                t![>] => {
                    parser.consume(t![<]);
                    Some(BinaryOperator::RightShift)
                },
                t![=] => {
                    parser.consume(t![=]);
                    Some(BinaryOperator::GreatherOrEqualThan)
                },
                _ => Some(BinaryOperator::GreaterThan)
            }
        },
        t![=] => {
            parser.consume(t![=]);
            match parser.current() {
                t![=] => {
                    parser.consume(t![=]);
                    Some(BinaryOperator::Equals)
                },
                _ => {
                    parser.missing(&[t![=]]);
                    Some(BinaryOperator::Equals)
                }
            }
        },
        t![!] => {
            parser.consume(t![!]);
            match parser.current() {
                t![=] => {
                    parser.consume(t![=]);
                    Some(BinaryOperator::NotEquals)
                },
                _ => {
                    parser.missing(&[t![=]]);
                    Some(BinaryOperator::NotEquals)
                }
            }
        },
        t![+] => {
            parser.consume(t![+]);
            Some(BinaryOperator::Add)
        },
        t![-] => {
            parser.consume(t![-]);
            Some(BinaryOperator::Subtract)
        },
        t![*] => {
            parser.consume(t![*]);
            match parser.current() {
                t![*] => {
                    parser.consume(t![*]);
                    Some(BinaryOperator::Exponent)
                },
                _ => {
                    Some(BinaryOperator::Multiply)
                }
            }
        },
        t![/] => {
            parser.consume(t![/]);
            Some(BinaryOperator::Divide)
        },
        t![%] => {
            parser.consume(t![%]);
            Some(BinaryOperator::Remainder)
        },
        t![&] => {
            parser.consume(t![&]);
            Some(BinaryOperator::BitwiseAnd)
        },
        t![|] => {
            parser.consume(t![|]);
            Some(BinaryOperator::BitwiseOr)
        },
        t![^] => {
            parser.consume(t![^]);
            Some(BinaryOperator::BitwiseXOr)
        },
        _ => None,
    }
}

fn parse_unary_operator(parser: &mut Parser) -> Option<UnaryOperator> {
    match parser.current() {
        t![+] => {
            parser.consume(t![+]);
            Some(UnaryOperator::KeepSign)
        },
        t![-] => {
            parser.consume(t![-]);
            Some(UnaryOperator::FlipSign)
        },
        t![!] => {
            parser.consume(t![!]);
            Some(UnaryOperator::BooleanNot)
        },
        t![~] => {
            parser.consume(t![~]);
            Some(UnaryOperator::BitwiseNot)
        },
        _ => None,
    }
}

fn is_at_expression_start(parser: &Parser) -> bool {
    match parser.current() {
        // parse an expression statement
        // if and loop expression start
        t![if] | t![loop] | t![while] | t![for] |
        // group expression start
        SyntaxKind::OpenParen |
        // primary expression start
        t![ident] | t![int] | t![float] | t![bool] | t![true] | t![false] |
        // unary operators
        t![+] | t![-] | t![!] | t![~] => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer;

    #[test]
    fn test_module_declaration() {
        let input = "mod foo;";
        let token = lexer::tokenize(input);
        let result = parse(&token, input);
        assert_eq!(result.diagnostics.len(), 0);
    }

    #[test]
    fn test_use_declaration() {
        let inputs = [
            "use foo;",
            "use foo::bar;",
            "use foo::{};",
            "use foo::{bar};",
            "use foo::{bar,};",
            "use foo::{bar, baz};",
            "use foo::{bar::foo, baz};",
            "use foo::{bar::{foo, bar}, baz};",
            "use foo::*;",
            "use foo::{bar::*};",
            "use foo::{bar::*,};",
            "use foo::{bar::*, baz::*};",
            "use foo::bar::baz::a::b::c;",
            "use foo::bar::baz::a::b::c::*;",
            "use package::foo;",
            "use package::*;",
            "use package::foo::*;",
        ];

        for input in inputs {
            let token = lexer::tokenize(input);
            let result = parse(&token, input);
            assert_eq!(result.diagnostics.len(), 0);
        }
    }

    #[test]
    fn test_struct_declaration() {
        let inputs = [
            "struct Foo{}",
            "struct Foo {}",
            "struct Foo { }",
            "struct Foo{foo:Bar}",
            "struct Foo { foo : Bar }",
            "struct Foo {foo:Bar,}",
            "struct Foo {foo:Bar,bar:Foo}",
            "struct Foo { foo: Bar, bar: Foo }",
            "struct Foo { foo: bar::Baz, bar: Foo }",
            "struct Foo { foo: bar::Baz, bar: foo::foo::Foo }",
        ];

        for input in inputs {
            let token = lexer::tokenize(input);
            let result = parse(&token, input);
            assert_eq!(result.diagnostics.len(), 0);
        }
    }

    #[test]
    fn test_fn_declaration() {
        let inputs = [
            "fn foo() {}",
            "fn foo(bar: Bar) {}",
            "fn foo(bar: bar::Bar) {}",
            "fn foo(bar: bar::foo::Bar, foo: foo::bar::Foo) {}",
            "fn foo() -> Foo {}",
            "fn foo() -> foo::Foo {}",
            "fn foo(a: A, b: B, c: C) -> foo::Foo {}",
        ];

        for input in inputs {
            let token = lexer::tokenize(input);
            let result = parse(&token, input);
            assert_eq!(result.diagnostics.len(), 0);
        }
    }

    #[test]
    fn test_expression_statements() {
        let inputs = [
            "fn foo() { true }",
            "fn foo() { false }",
            "fn foo() { 1.0 }",
            "fn foo() { 3 }",
            "fn foo() { 1 + 1 }",
            "fn foo() { 1 + 2 * 3 }",
            "fn foo() { 1 * 2 + 3 }",
            "fn foo() { true; false }",
            "fn foo() { a + b; b + c; }",
        ];

        for input in inputs {
            let token = lexer::tokenize(input);
            let result = parse(&token, input);
            assert_eq!(result.diagnostics.len(), 0);
        }
    }

    #[test]
    fn test_call_expression() {
        let inputs = [
            "fn foo() { some_ident(); }",
            "fn foo() { a(b); }",
            "fn foo() { a(b, c); }",
            "fn foo() { a(b, c,); }",
            "fn foo() { a(1 + 2, 3 * 4); }",
        ];

        for input in inputs {
            let token = lexer::tokenize(input);
            let result = parse(&token, input);
            assert_eq!(result.diagnostics.len(), 0);
        }
    }

    #[test]
    fn test_index_expression() {
        let inputs = [
            "fn foo() { a[b]; }",
            "fn foo() { a[1 + 2]; }",
            "fn foo() { a[0][1]; }",
        ];

        for input in inputs {
            let token = lexer::tokenize(input);
            let result = parse(&token, input);
            assert_eq!(result.diagnostics.len(), 0);
        }
    }

    #[test]
    fn test_dot_expression() {
        let inputs = [
            "fn foo() { a.b }",
            "fn foo() { a.b.c }",
        ];

        for input in inputs {
            let token = lexer::tokenize(input);
            let result = parse(&token, input);
            assert_eq!(result.diagnostics.len(), 0);
        }
    }

    #[test]
    fn test_unary_expression() {
        let inputs = [
            "fn foo() { -1 }",
            "fn foo() { +1 }",
            "fn foo() { !true }",
            "fn foo() { ~0 }",
        ];

        for input in inputs {
            let token = lexer::tokenize(input);
            let result = parse(&token, input);
            assert_eq!(result.diagnostics.len(), 0);
        }
    }
}
