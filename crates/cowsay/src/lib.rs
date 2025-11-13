//! The `cowsay` crate provides a library for generating cowsay-style ASCII art.
//!
//! This crate allows you to create custom cowsay messages with various options,
//! such as different cow files, eyes, tongues, and text wrapping.
//!
//! # Examples
//!
//! ```
//! use cowsay::CowsayOption;
//!
//! let message = "Hello, world!";
//! let cow_message = CowsayOption::builder()
//!     .with_message(message)
//!     .build()
//!     .unwrap()
//!     .parser()
//!     .unwrap()
//!     .think();
//!
//! println!("{}", cow_message);
//! ```

use crate::builder::CowsayOptionBuilder;

use cowsay_parser::Cow as CowParser;

/// Provides a builder for constructing `CowsayOption` instances.
pub mod builder;
/// Handles the loading and selection of cow files.
pub mod cows;
/// Defines error types for the `cowsay` crate.
pub mod errors;

/// Represents the options for generating a cowsay message.
///
/// Use the `CowsayOptionBuilder` to create instances of this struct.
#[derive(Debug)]
#[allow(clippy::struct_excessive_bools)]
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
    /// Creates a new `CowsayOptionBuilder` for constructing `CowsayOption` instances.
    ///
    /// # Examples
    ///
    /// ```
    /// use cowsay::CowsayOption;
    ///
    /// let builder = CowsayOption::builder();
    /// ```
    pub fn builder() -> CowsayOptionBuilder<'a> {
        CowsayOptionBuilder::default()
    }

    /// Parses the `CowsayOption` into a `CowParser` instance.
    ///
    /// This method applies the configured options (e.g., cow file, eyes, tongue,
    /// wrapping) to create a `CowParser` ready to generate the cowsay message.
    ///
    /// # Returns
    ///
    /// A `Result` which is:
    /// - `Ok(CowParser)` if the options are successfully parsed and a `CowParser`
    ///   can be created.
    /// - `Err(CowsayError)` if there is an issue loading a cow file or parsing
    ///   its template.
    ///
    /// # Panics
    ///
    /// This method panics if `wrap_column` is set but `wrap` is false,
    /// as `unwrap()` is called on `self.wrap_column` without a preceding check
    /// that `self.wrap` is true. This scenario should ideally be prevented
    /// by the `CowsayOptionBuilder` ensuring consistent state.
    ///
    /// # Examples
    ///
    /// ```
    /// use cowsay::CowsayOption;
    ///
    /// let parser = CowsayOption::builder()
    ///     .with_message("Hello")
    ///     .build()
    ///     .unwrap()
    ///     .parser()
    ///     .unwrap();
    /// ```
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
            let file_name = format!("{file}.cow");
            template = cows::get_cow_from_file(file_name.as_str())?;
        }

        if template.is_empty() {
            Ok(parser.build(None))
        } else {
            Ok(parser.build_with_template(&template)?)
        }
    }
}
