use textwrap::wrap;

use crate::builder::CowBuilder;

mod builder;
mod errors;

#[derive(Debug)]
pub struct Cow {
    eyes: String,
    tongue: String,
    template: String,
    text: String,
    thoughts: String,
    thinking: bool,
    balloon_width: i8,
    word_wrap: bool,
}

impl Cow {
    pub fn builder() -> builder::CowBuilder {
        CowBuilder::default()
    }

    pub fn say(mut self, phrase: Option<&str>) -> String {
        // Placeholder implementation for generating the cow saying the text
        if let Some(value) = phrase {
            self.text = value.to_string();
        }
        let template = self.format_template();
        (self.generate_balloon() + &template).as_str().to_string()
    }

    fn format_template(&self) -> String {
        // Placeholder implementation for formatting the cow file
        self.template
            .replace("$eyes", &self.eyes)
            .replace("$tongue", &self.tongue)
            .replace("$thoughts", &self.thoughts)
    }

    fn generate_balloon(&self) -> String {
        let mut width = self.balloon_width;
        let mut line_count = 1;
        let is_multiline = self.word_wrap && self.text.len() > width as usize;
        if !is_multiline {
            width = self.text.len() as i8;
        } else {
            line_count = ((self.text.len() as i8 / width) + 1) as usize;
        }

        let top_border = format!(" {}\n", "_".repeat(width as usize + 2));
        let btm_border = format!(" {}\n", "-".repeat(width as usize + 2));

        let lines = wrap(&self.text, width as usize);

        let mut balloon_lines: Vec<String> = Vec::with_capacity(line_count + 2);
        balloon_lines.push(top_border);
        balloon_lines.push(self.format_balloon_line(
            &lines[0],
            true,
            !is_multiline,
        ));
        if is_multiline {
            for line in &lines[1..lines.len() - 1] {
                balloon_lines
                    .push(self.format_balloon_line(line, false, false));
            }
            balloon_lines.push(self.format_balloon_line(
                &lines[line_count - 1],
                false,
                true,
            ));
        }
        balloon_lines.push(btm_border);

        String::from_iter(balloon_lines)
    }

    fn format_balloon_line(
        &self,
        line: &str,
        is_first: bool,
        is_last: bool,
    ) -> String {
        let left_border = if is_first {
            if self.thinking {
                "("
            } else {
                "<"
            }
        } else if is_last {
            if self.thinking {
                ")"
            } else {
                ">"
            }
        } else {
            "|"
        };

        let right_border = if is_first {
            if self.thinking {
                ")"
            } else {
                ">"
            }
        } else if is_last {
            if self.thinking {
                "("
            } else {
                "<"
            }
        } else {
            "|"
        };

        format!("{} {} {}\n", left_border, line, right_border)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let cow = super::Cow::builder()
            .with_eyes("oo")
            .with_tongue("  ")
            .with_text("Hello, World!")
            .build();

        let output = cow.say(None);
        let expected_output = [
            " _______________\n",
            "< Hello, World! >\n",
            " ---------------\n",
        ];

        assert_eq!(String::from_iter(expected_output), output);
    }

    #[test]
    fn it_works_with_template() {
        let cow = super::Cow::builder()
            .with_eyes("oo")
            .with_tongue("  ")
            .with_thoughts(r"\")
            .with_template(
                [
                    "        $thoughts   ^__^",
                    r"         $thoughts  ($eyes)\_______",
                    r"            (__)\       )\/\",
                    r"             $tongue ||----w |",
                    r"                ||     ||",
                ]
                .join("\n")
                .as_str(),
            )
            .with_text("Hello world")
            .build();

        let output = cow.say(None);
        let expected_output = [
            r" _____________",
            r"< Hello world >",
            r" -------------",
            r"        \   ^__^",
            r"         \  (oo)\_______",
            r"            (__)\       )\/\",
            r"                ||----w |",
            r"                ||     ||",
        ]
        .join("\n");

        assert_eq!(expected_output, output);
    }
}
