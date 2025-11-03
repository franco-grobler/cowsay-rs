use std::{fs::File, io::Read, path::Path};

use regex::Regex;

use crate::errors::ParseError;

pub fn load_template(path: &Path) -> Result<String, ParseError> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn load_cow(raw: &str) -> Result<String, ParseError> {
    if raw.is_empty() {
        return Err(ParseError::InvalidTemplateFormat(
            "Empty template".to_string(),
        ));
    }
    let cow_re = Regex::new(r#"\$the_cow\s*=\s*<<"*EOC"*;*\n([\s\S]*)\nEOC"#)
        .expect("Regex is fucked");

    cow_re.captures(raw).map_or_else(
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
