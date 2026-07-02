/// Chops off the last character of a string and returns the character chopped.
///
/// * `val`: Value to chop
pub fn chop(mut val: String) -> String {
    if val.is_empty() {
        return String::new();
    }
    val.split_off(val.len() - 1)
}
#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::stdlib::chop::chop;

    #[test]
    fn can_chop() {
        assert_eq!(chop("hello".to_string()), "o");
    }

    #[test]
    fn can_chop_empty_strings() {
        assert_eq!(chop(String::new()), "");
    }
}
