use crate::syn::cst::SyntaxKind;

pub type TokenSet = std::collections::HashSet<SyntaxKind>;

pub fn token_set(tokens: &[SyntaxKind]) -> TokenSet {
    let mut set = TokenSet::with_capacity(tokens.len());
    for token in tokens {
        set.insert(*token);
    }
    set
}
