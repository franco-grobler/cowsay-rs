//! Template parsing errors.

use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug)]
/// Template parsing errors.
pub enum ParseError {
    /// File reading errors.
    IoError(std::io::Error),
    /// Template could not be found.
    TemplateNotFound(String, Vec<String>),
    /// Template could not be parsed.
    InvalidTemplateFormat(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::IoError(item) => {
                write!(f, "{item} could not be accessed")
            }
            Self::TemplateNotFound(item, locations) => {
                write!(
                    f,
                    "Template not found: {}\nSearched locations:\n {}",
                    item,
                    locations.join("\n")
                )
            }
            Self::InvalidTemplateFormat(item) => {
                write!(f, "Invalid template format: {item}")
            }
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::IoError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ParseError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}
