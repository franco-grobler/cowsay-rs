use cowsay_template::{
    CowTemplate, DEFAULT_EYES, DEFAULT_THOUGHTS, DEFAULT_TONGUE,
    errors::ParseError,
};
use std::path::Path;

use crate::Cow;

#[derive(Debug)]
pub struct CowBuilder {
    eyes: String,
    tongue: String,
    text: String,
    thoughts: String,
    thinking: bool,
    balloon_width: usize,
    word_wrap: bool,
}

impl Default for CowBuilder {
    fn default() -> Self {
        CowBuilder {
            eyes: DEFAULT_EYES.to_string(),
            tongue: DEFAULT_TONGUE.to_string(),
            text: "Hello World".to_string(),
            thoughts: DEFAULT_THOUGHTS.to_string(),
            thinking: false,
            balloon_width: 40,
            word_wrap: true,
        }
    }
}

impl CowBuilder {
    pub fn with_eyes(mut self, eyes: &str) -> Self {
        self.eyes = eyes.to_string();
        self
    }

    pub fn with_tongue(mut self, tongue: &str) -> Self {
        self.tongue = tongue.to_string();
        self
    }

    pub fn with_text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }

    pub fn with_thoughts(mut self, thoughts: &str) -> Self {
        self.thoughts = thoughts.to_string();
        self
    }

    pub fn with_thinking(mut self, thinking: bool) -> Self {
        self.thinking = thinking;
        self
    }

    pub fn with_balloon_width(mut self, width: usize) -> Self {
        self.balloon_width = width;
        self
    }

    pub fn with_word_wrapped(mut self, wrap: bool) -> Self {
        self.word_wrap = wrap;
        self
    }

    fn create_variable_map(&self) -> std::collections::HashMap<String, String> {
        let mut variables = std::collections::HashMap::new();
        variables.insert("eyes".to_string(), self.eyes.clone());
        variables.insert("tongue".to_string(), self.tongue.clone());
        variables.insert("thoughts".to_string(), self.thoughts.clone());
        variables
    }

    pub fn build_with_template(
        self,
        template: &str,
    ) -> Result<Cow, ParseError> {
        let template = CowTemplate::from_template(template)?;
        Ok(self.build(Some(template)))
    }

    pub fn build_with_template_from_file(
        self,
        file_path: &Path,
    ) -> Result<Cow, ParseError> {
        let template = CowTemplate::from_file(file_path)?;
        Ok(self.build(Some(template)))
    }

    pub fn build(self, template: Option<CowTemplate>) -> Cow {
        let mut set_template = template.unwrap_or_default();
        set_template.apply_variables(self.create_variable_map());
        Cow {
            template: set_template,
            text: self.text,
            thinking: self.thinking,
            balloon_width: self.balloon_width,
            word_wrap: self.word_wrap,
        }
    }
}

#[cfg(test)]
mod tests {
    use cowsay_template::CowTemplate;

    #[test]
    fn has_default_values() {
        let cow = super::Cow::builder().build(None);

        assert_eq!(cow.template.render(), CowTemplate::default().render());
        assert_eq!(cow.text, "Hello World");
        assert!(!cow.thinking);
        assert_eq!(cow.balloon_width, 40);
        assert!(cow.word_wrap);
    }

    #[test]
    fn it_works() {
        let cow = super::Cow::builder()
            .with_eyes("^^")
            .with_tongue("U ")
            .with_text("Custom text")
            .with_thoughts("*")
            .with_thinking(true)
            .with_balloon_width(50)
            .with_word_wrapped(false)
            .build(None);

        let rendered_template = cow.template.render();
        let expected_template = [
            r"        *   ^__^",
            r"         *  (^^)\_______",
            r"            (__)\       )\/\",
            r"             U  ||----w |",
            r"                ||     ||",
            "",
        ]
        .join("\n");

        assert_eq!(rendered_template, expected_template);
        assert_eq!(cow.text, "Custom text");
        assert!(cow.thinking);
        assert_eq!(cow.balloon_width, 50);
        assert!(!cow.word_wrap);
    }
}
