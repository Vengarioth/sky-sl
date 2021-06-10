pub fn is_num_literal_start(c: char) -> bool {
    ('0'..='9').contains(&c)
}

pub fn is_num_literal_continue(c: char) -> bool {
    ('0'..='9').contains(&c)
    || c == '.'
}
