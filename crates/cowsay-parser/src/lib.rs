use cowsay_template::CowTemplate;
use textwrap::wrap;

use crate::builder::CowBuilder;

mod builder;

#[derive(Debug)]
pub struct Cow<'a> {
    template: CowTemplate,
    text: &'a str,
    thinking: bool,
    balloon_width: usize,
    word_wrap: bool,
}

impl<'a> Cow<'a> {
    pub fn builder() -> builder::CowBuilder<'static> {
        CowBuilder::default()
    }

    pub fn say(mut self, phrase: Option<&'a str>) -> String {
        if let Some(value) = phrase {
            self.text = value;
        }
        self.generate_balloon() + &self.format_template()
    }

    fn format_template(self) -> String {
        self.template.render()
    }

    fn generate_balloon(&self) -> String {
        let mut width = self.balloon_width;
        let mut line_count = 1;
        let is_multiline = self.word_wrap && self.text.len() > width;
        if !is_multiline {
            width = self.text.len();
        } else {
            line_count = (self.text.len() / width) + 1;
        }

        let top_border =
            format!(" {}\n", "_".repeat(width + 2 * !is_multiline as usize));
        let btm_border =
            format!(" {}\n", "-".repeat(width + 2 * !is_multiline as usize));

        let lines = wrap(self.text, width);

        let mut balloon_lines: Vec<String> = Vec::with_capacity(line_count + 2);
        balloon_lines.push(top_border);
        balloon_lines.push(self.format_balloon_line(
            &lines[0],
            true,
            !is_multiline,
            is_multiline,
            width,
        ));
        if is_multiline {
            for line in &lines[1..lines.len() - 1] {
                balloon_lines.push(self.format_balloon_line(
                    line,
                    false,
                    false,
                    is_multiline,
                    width,
                ));
            }
            balloon_lines.push(self.format_balloon_line(
                &lines[line_count - 1],
                false,
                true,
                is_multiline,
                width,
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
        is_multiline: bool,
        width: usize,
    ) -> String {
        let left_border = if is_first {
            if self.thinking {
                "("
            } else if is_multiline {
                "/"
            } else {
                "<"
            }
        } else if is_last {
            if self.thinking {
                ")"
            } else if is_multiline {
                r"\"
            } else {
                ">"
            }
        } else {
            "|"
        };

        let right_border = if is_first {
            if self.thinking {
                ")"
            } else if is_multiline {
                r"\"
            } else {
                ">"
            }
        } else if is_last {
            if self.thinking {
                "("
            } else if is_multiline {
                "/"
            } else {
                "<"
            }
        } else {
            "|"
        };

        format!(
            "{} {:width$} {}\n",
            left_border,
            line,
            right_border,
            width = width - 2
        )
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_str_eq;

    #[test]
    fn it_works_with_template() {
        let template = [
            r#"$the_cow = <<"EOC";"#,
            "        $thoughts   ^__^",
            r"         $thoughts  ($eyes)\_______",
            r"            (__)\       )\/\",
            r"             $tongue ||----w |",
            r"                ||     ||",
            "EOC",
        ]
        .join("\n");

        let cow = super::Cow::builder()
            .with_eyes("oo")
            .with_tongue("  ")
            .with_thoughts(r"\")
            .with_text("Hello world")
            .build_with_template(template)
            .expect("Could not parse template");

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
            "",
        ]
        .join("\n");

        assert_str_eq!(expected_output, output);
    }
}
