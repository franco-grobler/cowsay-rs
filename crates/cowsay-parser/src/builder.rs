use cowsay_template::{
    CowTemplate, DEFAULT_EYES, DEFAULT_THOUGHTS, DEFAULT_TONGUE,
    errors::ParseError,
};
use std::{collections::HashMap, path::Path};

use crate::Cow;

#[derive(Debug)]
pub struct CowBuilder<'a> {
    eyes: &'a str,
    tongue: &'a str,
    text: &'a str,
    thoughts: &'a str,
    thinking: bool,
    balloon_width: usize,
    word_wrap: bool,
}

impl Default for CowBuilder<'_> {
    fn default() -> Self {
        CowBuilder {
            eyes: DEFAULT_EYES,
            tongue: DEFAULT_TONGUE,
            text: "Hello World",
            thoughts: DEFAULT_THOUGHTS,
            thinking: false,
            balloon_width: 40,
            word_wrap: true,
        }
    }
}

impl<'a> CowBuilder<'a> {
    pub const fn with_eyes(mut self, eyes: &'a str) -> Self {
        self.eyes = eyes;
        self
    }

    pub const fn with_tongue(mut self, tongue: &'a str) -> Self {
        self.tongue = tongue;
        self
    }

    pub const fn with_text(mut self, text: &'a str) -> Self {
        self.text = text;
        self
    }

    pub const fn with_thoughts(mut self, thoughts: &'a str) -> Self {
        self.thoughts = thoughts;
        self
    }

    pub const fn with_thinking(mut self, thinking: bool) -> Self {
        self.thinking = thinking;
        self
    }

    pub const fn with_balloon_width(mut self, width: usize) -> Self {
        self.balloon_width = width;
        self
    }

    pub const fn with_word_wrapped(mut self, wrap: bool) -> Self {
        self.word_wrap = wrap;
        self
    }

    fn create_variable_map(&self) -> HashMap<String, String> {
        HashMap::from([
            ("eyes".to_string(), self.eyes.to_string()),
            ("tongue".to_string(), self.tongue.to_string()),
            ("text".to_string(), self.text.to_string()),
            ("thoughts".to_string(), self.thoughts.to_string()),
        ])
    }

    pub fn build_with_template(
        self,
        template: &str,
    ) -> Result<Cow<'a>, ParseError> {
        let template = CowTemplate::from_template(template)?;
        Ok(self.build(Some(template)))
    }

    pub fn build_with_template_from_file(
        self,
        file_path: &'a Path,
    ) -> Result<Cow<'a>, ParseError> {
        let template = CowTemplate::from_file(file_path)?;
        Ok(self.build(Some(template)))
    }

    pub fn build(self, template: Option<CowTemplate>) -> Cow<'a> {
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
