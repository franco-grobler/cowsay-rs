use std::{fs::File, io::Read, path::Path};

use crate::{errors::ParseError, patterns};

/// Reads the content of a file to a string.
///
/// * `path`: Path of the file to read.
pub fn load_template(path: &Path) -> Result<String, ParseError> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

/// Extracts the cow definition from a raw template string.
///
/// Strips escape characters before processing.
///
/// * `raw`: Cow template string.
pub fn load_cow(raw: &str) -> Result<String, ParseError> {
    if raw.is_empty() {
        return Err(ParseError::InvalidTemplateFormat(
            "Empty template".to_string(),
        ));
    }
    let stripped = strip_escape_characters(raw);

    let cow_re = patterns::get_cow_regex();

    cow_re.captures(stripped.as_str()).map_or_else(
        || {
            Err(ParseError::InvalidTemplateFormat(
                "Template does not match cow format".to_string(),
            ))
        },
        |caps| {
            let cow_content = caps.get(1).map_or("", |m| m.as_str());
            Ok(cow_content.to_string())
        },
    )
}

/// Removes escape characters from a string.
///
/// * `text`: string to process.
fn strip_escape_characters(text: &str) -> String {
    patterns::get_substitution_regex()
        .replace_all(text, "$1")
        .to_string()
}

pub fn find_template(
    name: &str,
    search_paths: &[&Path],
) -> Result<String, ParseError> {
    // Placeholder for finding template in given search paths
    for path in search_paths {
        let candidate = path.join(name);
        if candidate.exists() {
            return load_template(&candidate);
        }
    }

    Err(ParseError::TemplateNotFound(
        name.to_string(),
        search_paths
            .iter()
            .map(|p| p.display().to_string())
            .collect(),
    ))
}
