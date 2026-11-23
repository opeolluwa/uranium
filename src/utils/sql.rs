pub fn escape_like(raw: &str) -> String {
    let mut escaped = String::with_capacity(raw.len());

    for character in raw.chars() {
        if matches!(character, '%' | '_' | '\\') {
            escaped.push('\\');
        }
        escaped.push(character);
    }

    escaped
}

pub fn contains_ilike(term: &str) -> String {
    format!("%{}%", escape_like(term.trim()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escapes_like_wildcards() {
        assert_eq!(escape_like("50%"), "50\\%");
        assert_eq!(escape_like("a_b"), "a\\_b");
        assert_eq!(escape_like("back\\slash"), "back\\\\slash");
    }

    #[test]
    fn leaves_ordinary_terms_untouched() {
        assert_eq!(escape_like("shirt"), "shirt");
        assert_eq!(escape_like(""), "");
        assert_eq!(escape_like("50% off"), "50\\% off");
    }

    #[test]
    fn escapes_non_ascii_without_losing_characters() {
        assert_eq!(escape_like("àshirt_"), "àshirt\\_");
    }

    #[test]
    fn wraps_the_escaped_term_in_wildcards() {
        assert_eq!(contains_ilike("chair"), "%chair%");
        assert_eq!(contains_ilike("  50% off  "), "%50\\% off%");
    }
}
