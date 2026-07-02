// Lexer utilities.

/// Check is a char is valid for an identifier.
///
/// * `c`: Character to check.
pub(crate) const fn is_valid_identifier(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

#[cfg(test)]
mod tests {
    use crate::lexer::utils::is_valid_identifier;

    #[test]
    fn test_is_valid_identifier() {
        let table = [
            ('A', true),
            ('a', true),
            ('C', true),
            ('c', true),
            ('_', true),
            (';', false),
            (' ', false),
            ('-', false),
            ('$', false),
            ('\n', false),
            ('\t', false),
        ];
        for test in table {
            assert_eq!(is_valid_identifier(test.0), test.1);
        }
    }
}
