use crate::builder::CowsayOptionBuilder;

use cowsay_parser::Cow as CowParser;

pub mod builder;
pub mod cows;
pub mod errors;

#[derive(Debug)]
pub struct CowsayOption {
    borg: bool,
    dead: bool,
    greedy: bool,
    sleepy: bool,
    tired: bool,
    wired: bool,
    young: bool,
    file: Option<String>,
    random: bool,
    eyes: Option<String>,
    tongue: Option<String>,
    wrap: bool,
    wrap_column: Option<usize>,
}

impl CowsayOption {
    pub fn builder() -> CowsayOptionBuilder {
        CowsayOptionBuilder::default()
    }

    pub fn parser(self) -> Result<CowParser, errors::CowsayError> {
        let mut template: Option<String> = None;
        let mut parser = CowParser::builder();
        if self.borg {
            parser = parser.with_eyes("==");
        } else if self.dead {
            parser = parser.with_eyes("xx").with_tongue("U ");
        } else if self.greedy {
            parser = parser.with_eyes("$$");
        } else if self.sleepy {
            parser = parser.with_eyes("**").with_tongue("U ");
        } else if self.tired {
            parser = parser.with_eyes("--");
        } else if self.wired {
            parser = parser.with_eyes("OO");
        } else if self.young {
            parser = parser.with_eyes("..");
        } else if let Some(eyes) = &self.eyes {
            parser = parser.with_eyes(eyes);
        } else if let Some(tongue) = &self.tongue {
            parser = parser.with_tongue(tongue);
        } else if self.random {
            template = Some(cows::get_random_cow());
        } else if let Some(file) = &self.file {
            let file_name = format!("{}.cow", file);
            template = Some(cows::get_cow_from_file(file_name.as_str())?);
        }

        if self.wrap {
            parser = parser.with_word_wrapped(true);
        }
        if self.wrap_column.is_some() {
            parser = parser.with_balloon_width(self.wrap_column.unwrap() as i8);
        }

        if template.is_none() {
            Ok(parser.build(None))
        } else {
            Ok(parser.build_with_template(template.unwrap().as_str())?)
        }
    }
}
