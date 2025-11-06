use std::sync::OnceLock;

use regex::Regex;

pub fn get_description_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();

    RE.get_or_init(|| Regex::new(r"^##(.*)").unwrap())
}

pub fn get_variable_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();

    RE.get_or_init(|| Regex::new(r"\$\{?(\w+)}?").unwrap())
}
