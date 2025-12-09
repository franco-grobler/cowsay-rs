//! Regular expressions used in template parsing.

use std::sync::OnceLock;

use regex::Regex;

/// Regex for extracting the cow from the cowfile.
pub(crate) fn get_cow_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();

    RE.get_or_init(|| {
        Regex::new(r#"\$the_cow\s*=\s*<<"*EOC"*;*\n([\s\S]*)\nEOC"#).unwrap()
    })
}

/// Regex for extracting the description from the cowfile.
pub(crate) fn get_description_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();

    RE.get_or_init(|| Regex::new(r"(?m)^##\s?(.*)").unwrap())
}

/// Regex for matching escaped special characters in the template.
pub(crate) fn get_substitution_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();

    RE.get_or_init(|| {
        Regex::new(r"\\([\\\.\+\*\?\(\)\|\[\]\{\}\^\$\#&\-~@])").unwrap()
    })
}

/// Regex for matching variable placeholders in the template.
pub(crate) fn get_variable_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();

    RE.get_or_init(|| Regex::new(r"\$\{?(\w+)}?").unwrap())
}
