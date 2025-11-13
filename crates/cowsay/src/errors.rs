//! Crate specific errors.

use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;

use cowsay_template::errors::ParseError;

#[derive(Debug)]
/// Cowsay specific errors.
pub enum CowsayError {
    /// Unable to find cowfile.
    CowfileNotFound(String),
    /// Could not parse cowfile.
    CowfileParseError(ParseError),
}

impl Display for CowsayError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::CowfileNotFound(item) => {
                write!(f, "Could not find cowfile for {item}")
            }
            Self::CowfileParseError(item) => {
                write!(f, "Could not parse cowfile: {item}")
            }
        }
    }
}

impl Error for CowsayError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::CowfileParseError(e) => Some(e),
            Self::CowfileNotFound(_) => None,
        }
    }
}

impl From<ParseError> for CowsayError {
    fn from(err: ParseError) -> Self {
        Self::CowfileParseError(err)
    }
}
