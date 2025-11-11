use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum ParseError {
    IoError(std::io::Error),
    TemplateNotFound(String, Vec<String>),
    InvalidTemplateFormat(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            ParseError::IoError(item) => {
                write!(f, "{} could not be accessed", item)
            }
            ParseError::TemplateNotFound(item, locations) => {
                write!(
                    f,
                    "Template not found: {}\nSearched locations:\n {}",
                    item,
                    locations.join("\n")
                )
            }
            ParseError::InvalidTemplateFormat(item) => {
                write!(f, "Invalid template format: {}", item)
            }
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParseError::IoError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ParseError {
    fn from(err: std::io::Error) -> Self {
        ParseError::IoError(err)
    }
}
