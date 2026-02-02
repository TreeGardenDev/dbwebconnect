pub fn is_valid_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if is_ident_start(c) => {}
        _ => return false,
    }

    for c in chars {
        if !is_ident_char(c) {
            return false;
        }
    }

    true
}

fn is_ident_start(c: char) -> bool {
    c == '_' || c.is_ascii_alphabetic()
}

fn is_ident_char(c: char) -> bool {
    c == '_' || c.is_ascii_alphanumeric()
}

pub fn escape_sql_literal(value: &str) -> String {
    value.replace('\'', "''")
}
