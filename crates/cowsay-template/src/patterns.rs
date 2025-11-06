use std::sync::OnceLock;

use regex::Regex;

pub fn get_cow_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();

    RE.get_or_init(|| {
        Regex::new(r#"\$the_cow\s*=\s*<<"*EOC"*;*\n([\s\S]*)\nEOC"#).unwrap()
    })
}

pub fn get_description_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();

    RE.get_or_init(|| Regex::new(r"^##(.*)").unwrap())
}

pub fn get_substitution_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();

    RE.get_or_init(|| {
        Regex::new(r#"\\([\\\.\+\*\?\(\)\|\[\]\{\}\^\$\#&\-~@])"#).unwrap()
    })
}

pub fn get_variable_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();

    RE.get_or_init(|| Regex::new(r"\$\{?(\w+)}?").unwrap())
}
