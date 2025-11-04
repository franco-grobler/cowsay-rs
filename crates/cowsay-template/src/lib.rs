use std::collections::HashMap;
use std::path::Path;

use crate::errors::ParseError;
use crate::loader::{load_cow, load_template};

pub mod errors;
mod loader;

pub const DEFAUL_COW: &str = include_str!("../../../cows/default.cow");
pub const DEFAULT_EYES: &str = "oo";
pub const DEFAULT_THOUGHTS: &str = r"\";
pub const DEFAULT_TONGUE: &str = "  ";

#[derive(Debug)]
pub struct CowTemplate {
    raw_content: String,
    cow: String,
    variables: HashMap<String, String>,
}

#[derive(Debug)]
pub struct CowTemplateResult {
    pub rendered: String,
    pub description: String,
}

impl Default for CowTemplate {
    fn default() -> Self {
        let raw_content = DEFAUL_COW.to_string();
        let cow =
            load_cow(raw_content.as_str()).expect("Loading default cow failed");
        CowTemplate {
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
    pub fn from_file(
        path: &Path,
        variables: HashMap<String, String>,
    ) -> Result<Self, ParseError> {
        let raw_content = load_template(path)?;
        let cow = load_cow(raw_content.as_str())?;
        Ok(CowTemplate {
            variables,
            raw_content,
            cow,
        })
    }

    pub fn from_template(
        template: &str,
        variables: HashMap<String, String>,
    ) -> Result<Self, ParseError> {
        let raw_content = template.to_string();
        let cow = load_cow(raw_content.as_str())?;
        Ok(CowTemplate {
            variables,
            raw_content,
            cow,
        })
    }

    pub fn render(self) -> String {
        let variable_regex =
            regex::Regex::new(r"\$\{?(\w+)}?").expect("Regex is fucked");
        let mut rendered_content = self.cow.clone();

        variable_regex
            .captures_iter(rendered_content.clone().as_str())
            .for_each(|cap| {
                let instance_name = cap.get(0);
                if let Some(var_name) = cap.get(1) {
                    let var_name_str = var_name.as_str();
                    if let Some(value) = self.variables.get(var_name_str) {
                        rendered_content = rendered_content
                            .replace(instance_name.unwrap().as_str(), value);
                    }
                }
            });
        rendered_content
    }

    pub fn render_with_description(self) -> CowTemplateResult {
        let description_regex =
            regex::Regex::new(r"^##(.*)").expect("Regex is fucked");
        let description = if let Some(caps) =
            description_regex.captures(self.raw_content.as_str())
        {
            caps.get(1).map_or("", |m| m.as_str()).trim().to_string()
        } else {
            String::from("No description available")
        };
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
        ]
        .join("\n");

        println!("Output:\n{}", output);
        println!("Expected Output:\n{}", expected_output);
        assert_eq!(expected_output, output);
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
        let cow =
            CowTemplate::from_template(lines.join("\n").as_str(), variables)
                .expect("Creating CowTemplate failed");

        let output = cow.render();
        let expected_output = [
            r"        \   ^__^",
            r"         \  (oo)\_______",
            r"            (__)\       )\/\",
            r"                ||----w |",
            r"                ||     ||",
        ]
        .join("\n");

        assert_eq!(expected_output, output);
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
            writeln!(&cow_file, "{}", line).unwrap();
        }

        let variables = HashMap::from([
            ("thoughts".to_string(), r"\".to_string()),
            ("eyes".to_string(), "oo".to_string()),
            ("tongue".to_string(), "  ".to_string()),
        ]);
        let cow = CowTemplate::from_file(test_cow.as_path(), variables)
            .expect("Creating CowTemplate failed");

        let output = cow.render();
        let expected_output = [
            r"        \   ^__^",
            r"         \  (oo)\_______",
            r"            (__)\       )\/\",
            r"                ||----w |",
            r"                ||     ||",
        ]
        .join("\n");

        assert_eq!(expected_output, output);

        drop(cow_file);
        temp_dir.close().unwrap();
    }
}
