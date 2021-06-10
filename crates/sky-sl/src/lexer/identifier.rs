/// from https://github.com/rust-lang/rust/blob/master/compiler/rustc_lexer/src/lib.rs#L267
pub fn is_identifier_start(c: char) -> bool {
    ('a'..='z').contains(&c)
    || ('A'..='Z').contains(&c)
    || c == '_'
    || (c > '\x7f' && unicode_xid::UnicodeXID::is_xid_start(c))
}

/// https://github.com/rust-lang/rust/blob/master/compiler/rustc_lexer/src/lib.rs#L279
pub fn is_identifier_continue(c: char) -> bool {
    ('a'..='z').contains(&c)
    || ('A'..='Z').contains(&c)
    || ('0'..='9').contains(&c)
    || c == '_'
    || (c > '\x7f' && unicode_xid::UnicodeXID::is_xid_start(c))
}
