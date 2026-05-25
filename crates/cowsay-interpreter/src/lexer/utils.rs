// Lexer utilities.

/// Check is a char is valid for an identifier.
///
/// * `c`: Character to check.
pub(crate) const fn is_valid_identifier(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}
