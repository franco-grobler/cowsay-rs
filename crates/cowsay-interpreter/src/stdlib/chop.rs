/// Chops off the last character of a string and returns the character chopped.
///
/// * `val`: Value to chop
pub fn chop(mut val: String) -> String {
    val.split_off(val.len() - 1)
}
