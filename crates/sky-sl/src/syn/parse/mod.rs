use crate::lexer::Token;

mod diagnostic;
mod parser;

pub use diagnostic::*;
use parser::*;

use super::cst::SyntaxKind;

macro_rules! t {
    [:] => {$crate::syn::cst::SyntaxKind::Colon};
    [;] => {$crate::syn::cst::SyntaxKind::Semicolon};
    [,] => {$crate::syn::cst::SyntaxKind::Comma};
    [*] => {$crate::syn::cst::SyntaxKind::Star};
    [-] => {$crate::syn::cst::SyntaxKind::Minus};
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
    [package] => {$crate::syn::cst::SyntaxKind::PackageKeyword};
    [ident] => {$crate::syn::cst::SyntaxKind::Identifier};
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
    parser.expect(t!['}'], &[]);
    parser.end_node();
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
            dbg!(&result.diagnostics);
            dbg!(result.tree());
            assert_eq!(result.diagnostics.len(), 0);
        }

        panic!();
    }
}
