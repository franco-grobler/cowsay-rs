use std::{fs::File, io::Read, path::Path};

use crate::{errors::ParseError, Cow};

#[derive(Debug)]
pub struct CowBuilder {
    eyes: String,
    tongue: String,
    template: String,
    text: String,
    thoughts: String,
    thinking: bool,
    balloon_width: i8,
    word_wrap: bool,
}

impl Default for CowBuilder {
    fn default() -> Self {
        CowBuilder {
            eyes: "oo".to_string(),
            tongue: "  ".to_string(),
            template: "".to_string(),
            text: "Hello World".to_string(),
            thoughts: "o".to_string(),
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

    pub fn with_template(mut self, template: &str) -> Self {
        self.template = template.to_string();
        self
    }

    pub fn with_template_from_file(
        mut self,
        file_path: &Path,
    ) -> Result<Self, ParseError> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        self.template = contents;

        Ok(self)
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

    pub fn with_balloon_width(mut self, width: i8) -> Self {
        self.balloon_width = width;
        self
    }

    pub fn with_word_wrapped(mut self, wrap: bool) -> Self {
        self.word_wrap = wrap;
        self
    }

    pub fn build(self) -> Cow {
        Cow {
            eyes: self.eyes,
            tongue: self.tongue,
            template: self.template,
            text: self.text,
            thoughts: self.thoughts,
            thinking: self.thinking,
            balloon_width: self.balloon_width,
            word_wrap: self.word_wrap,
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn has_default_values() {
        let cow = super::Cow::builder().build();

        assert_eq!(cow.eyes, "oo");
        assert_eq!(cow.tongue, "  ");
        assert_eq!(cow.template, "");
        assert_eq!(cow.text, "Hello World");
        assert_eq!(cow.thoughts, "o");
        assert!(!cow.thinking);
        assert_eq!(cow.balloon_width, 40);
        assert!(cow.word_wrap);
    }

    #[test]
    fn it_works() {
        let cow = super::Cow::builder()
            .with_eyes("^^")
            .with_tongue("U ")
            .with_template("a cow template")
            .with_text("Custom text")
            .with_thoughts("*")
            .with_thinking(true)
            .with_balloon_width(50)
            .with_word_wrapped(false)
            .build();

        assert_eq!(cow.eyes, "^^");
        assert_eq!(cow.tongue, "U ");
        assert_eq!(cow.template, "a cow template");
        assert_eq!(cow.text, "Custom text");
        assert_eq!(cow.thoughts, "*");
        assert!(cow.thinking);
        assert_eq!(cow.balloon_width, 50);
        assert!(!cow.word_wrap);
    }
}
