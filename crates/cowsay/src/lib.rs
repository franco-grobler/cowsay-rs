use crate::builder::CowsayOptionBuilder;

use cowsay_parser::Cow as CowParser;

pub mod builder;
pub mod cows;
pub mod errors;

#[derive(Debug)]
pub struct CowsayOption<'a> {
    borg: bool,
    dead: bool,
    greedy: bool,
    sleepy: bool,
    tired: bool,
    wired: bool,
    young: bool,
    file: Option<&'a str>,
    random: bool,
    eyes: Option<&'a str>,
    tongue: Option<&'a str>,
    wrap: bool,
    wrap_column: Option<usize>,
}

impl<'a> CowsayOption<'a> {
    pub fn builder() -> CowsayOptionBuilder<'a> {
        CowsayOptionBuilder::default()
    }

    pub fn parser(self) -> Result<CowParser<'a>, errors::CowsayError> {
        let mut template: String = String::new();
        let mut parser = CowParser::builder();
        let (eyes, tongue) = if self.borg {
            (Some("=="), None)
        } else if self.dead {
            (Some("xx"), Some("U "))
        } else if self.greedy {
            (Some("$$"), None)
        } else if self.sleepy {
            (Some("**"), Some("U "))
        } else if self.tired {
            (Some("--"), None)
        } else if self.wired {
            (Some("OO"), None)
        } else if self.young {
            (Some(".."), None)
        } else {
            (self.eyes, self.tongue)
        };

        if let Some(e) = eyes {
            parser = parser.with_eyes(e);
        }
        if let Some(t) = tongue {
            parser = parser.with_tongue(t);
        }

        parser = parser.with_word_wrapped(self.wrap);
        if self.wrap_column.is_some() {
            parser = parser.with_balloon_width(self.wrap_column.unwrap());
        }

        if self.random {
            template = cows::get_random_cow()?;
        } else if let Some(file) = &self.file {
            let file_name = format!("{}.cow", file);
            template = cows::get_cow_from_file(file_name.as_str())?;
        }

        if template.is_empty() {
            Ok(parser.build(None))
        } else {
            Ok(parser.build_with_template(&template)?)
        }
    }
}
