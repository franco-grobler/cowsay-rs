//! Cowfile template parser and renderer.

use std::collections::HashMap;
use std::path::Path;

use crate::errors::ParseError;
use crate::loader::{load_cow, load_template};

pub mod errors;
pub mod loader;
pub mod patterns;

/// Default cowfile template content.
pub const DEFAULT_COW: &str = include_str!("../../../cows/default.cow");
/// Default eyes variable.
pub const DEFAULT_EYES: &str = "oo";
/// Default thoughts variable.
pub const DEFAULT_THOUGHTS: &str = r"\";
/// Default tongue variable.
pub const DEFAULT_TONGUE: &str = "  ";

#[derive(Debug)]
/// Represents a cowfile template that can be rendered with variables.
///
/// * `raw_content`: Raw template content as a string.
/// * `cow`: Extracted cow from cowfile content.
/// * `variables`: Variables to be applied in the template.
pub struct CowTemplate {
    raw_content: String,
    cow: String,
    variables: HashMap<String, String>,
}

#[derive(Debug)]
/// Results of rendering a cowfile template.
pub struct CowTemplateResult {
    /// Rendered cow.
    pub rendered: String,
    /// Top level comments of cowfile.
    pub description: String,
}

impl Default for CowTemplate {
    fn default() -> Self {
        let raw_content = DEFAULT_COW.to_string();
        let cow =
            load_cow(raw_content.as_str()).expect("Loading default cow failed");
        Self {
            raw_content,
            cow,
            variables: HashMap::from([
                ("eyes".to_string(), DEFAULT_EYES.to_string()),
                ("thoughts".to_string(), DEFAULT_THOUGHTS.to_string()),
                ("tongue".to_string(), DEFAULT_TONGUE.to_string()),
            ]),
        }
    }
}

impl CowTemplate {
    /// Creates a ``CowTemplate`` from a file path.
    ///
    /// * `path`: Path to the cowfile template.
    pub fn from_file(path: &Path) -> Result<Self, ParseError> {
        let raw_content = load_template(path)?;
        let cow = load_cow(raw_content.as_str())?;
        Ok(Self {
            raw_content,
            cow,
            variables: HashMap::from([
                ("eyes".to_string(), DEFAULT_EYES.to_string()),
                ("thoughts".to_string(), DEFAULT_THOUGHTS.to_string()),
                ("tongue".to_string(), DEFAULT_TONGUE.to_string()),
            ]),
        })
    }

    /// Creates a ``CowTemplate`` from a template string.
    ///
    /// * `template`: Raw cowfile template string.
    pub fn from_template(template: &str) -> Result<Self, ParseError> {
        let raw_content = template.to_string();
        let cow = load_cow(raw_content.as_str())?;
        Ok(Self {
            raw_content,
            cow,
            variables: HashMap::from([
                ("eyes".to_string(), DEFAULT_EYES.to_string()),
                ("thoughts".to_string(), DEFAULT_THOUGHTS.to_string()),
                ("tongue".to_string(), DEFAULT_TONGUE.to_string()),
            ]),
        })
    }

    /// Updates the variables used in the template.
    ///
    /// * `variables`: New variables to apply.
    pub fn apply_variables(&mut self, variables: HashMap<String, String>) {
        self.variables = variables;
    }

    /// Renders the cowfile template with the applied variables.
    pub fn render(self) -> String {
        let variable_regex = patterns::get_variable_regex();
        let mut rendered_content = self.cow.clone();

        rendered_content = variable_regex
            .replace_all(&rendered_content, |cap: &regex::Captures| {
                self.variables
                    .get(&cap[1])
                    .map_or_else(|| cap[0].to_string(), Clone::clone)
            })
            .into_owned();
        rendered_content + "\n"
    }

    /// Renders the cowfile template with the applied variables and extracts the description.
    pub fn render_with_description(self) -> CowTemplateResult {
        let description_regex = patterns::get_description_regex();
        let description = description_regex
            .captures(self.raw_content.as_str())
            .map_or_else(String::new, |caps| {
                caps.get(1).map_or("", |m| m.as_str()).trim().to_string()
            });
        CowTemplateResult {
            rendered: self.render(),
            description,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Write};

    use super::*;
    use pretty_assertions::assert_str_eq;
    use tempfile::tempdir;

    #[test]
    fn it_works_with_default() {
        let cow = CowTemplate::default();
        let output = cow.render();
        let expected_output = [
            r"        \   ^__^",
            r"         \  (oo)\_______",
            r"            (__)\       )\/\",
            r"                ||----w |",
            r"                ||     ||",
            "",
        ]
        .join("\n");

        assert_str_eq!(expected_output, output);
    }

    #[test]
    fn it_works_with_template() {
        let lines = [
            r#"$the_cow = <<"EOC";"#,
            "        $thoughts   ^__^",
            r"         $thoughts  ($eyes)\_______",
            r"            (__)\       )\/\",
            r"             $tongue ||----w |",
            r"                ||     ||",
            "EOC",
        ];

        let variables = HashMap::from([
            ("thoughts".to_string(), r"\".to_string()),
            ("eyes".to_string(), "oo".to_string()),
            ("tongue".to_string(), "  ".to_string()),
        ]);
        let mut cow = CowTemplate::from_template(lines.join("\n").as_str())
            .expect("Creating CowTemplate failed");
        cow.apply_variables(variables);

        let output = cow.render();
        let expected_output = [
            r"        \   ^__^",
            r"         \  (oo)\_______",
            r"            (__)\       )\/\",
            r"                ||----w |",
            r"                ||     ||",
            "",
        ]
        .join("\n");

        assert_str_eq!(expected_output, output);
    }

    #[test]
    fn it_works_with_file() {
        let temp_dir = tempdir().unwrap();
        let test_cow = temp_dir.path().join("test.cow");
        let cow_file = File::create(&test_cow).unwrap();
        let lines = [
            r#"$the_cow = <<"EOC";"#,
            "        $thoughts   ^__^",
            r"         $thoughts  ($eyes)\_______",
            r"            (__)\       )\/\",
            r"             $tongue ||----w |",
            r"                ||     ||",
            "EOC",
        ];
        for line in lines {
            writeln!(&cow_file, "{line}").unwrap();
        }

        let variables = HashMap::from([
            ("thoughts".to_string(), r"\".to_string()),
            ("eyes".to_string(), "oo".to_string()),
            ("tongue".to_string(), "  ".to_string()),
        ]);
        let mut cow = CowTemplate::from_file(test_cow.as_path())
            .expect("Creating CowTemplate failed");
        cow.apply_variables(variables);

        let output = cow.render();
        let expected_output = [
            r"        \   ^__^",
            r"         \  (oo)\_______",
            r"            (__)\       )\/\",
            r"                ||----w |",
            r"                ||     ||",
            "",
        ]
        .join("\n");

        assert_str_eq!(expected_output, output);

        drop(cow_file);
        temp_dir.close().unwrap();
    }
}
