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
    ['{'] => {$crate::syn::cst::SyntaxKind::OpenBrace};
    ['}'] => {$crate::syn::cst::SyntaxKind::CloseBrace};
    [mod] => {$crate::syn::cst::SyntaxKind::ModKeyword};
    [use] => {$crate::syn::cst::SyntaxKind::UseKeyword};
    [fn] => {$crate::syn::cst::SyntaxKind::FnKeyword};
    [struct] => {$crate::syn::cst::SyntaxKind::StructKeyword};
    [package] => {$crate::syn::cst::SyntaxKind::PackageKeyword};
    [ident] => {$crate::syn::cst::SyntaxKind::Identifier};
}

pub fn parse<'a>(token: &'a [Token], input: &'a str) {
    let mut parser = Parser::new(token, input);
    parse_module(&mut parser);
    dbg!(parser.finish::<crate::syn::ast::Root>());
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

}

/// parses a use group, e.g. `{module_a, module_b::c}`
fn parse_use_group(parser: &mut Parser) {

}

/// parses a use segment, meaning an identifier or a `package` keyword
fn parse_use_segment(parser: &mut Parser) {
    parser.expect_any(&[t![ident], t![package]], &[t!['{'], t!['}'], t![,], t![;], t![mod], t![use], t![fn], t![struct]]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer;

    #[test]
    fn test_module_declaration() {
        let input = "mod - mod foo;";
        let token =lexer::tokenize(input);
        parse(&token, input);
    }
}
