use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;

use cowsay_template::errors::ParseError;

#[derive(Debug)]
pub enum CowsayError {
    CowfileNotFound(String),
    CowfileParseError(ParseError),
}

impl Display for CowsayError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            CowsayError::CowfileNotFound(item) => {
                write!(f, "Could not find cowfile  for {}", item)
            }
            CowsayError::CowfileParseError(item) => {
                write!(f, "Could not parse cowfile: {}", item)
            }
        }
    }
}

impl Error for CowsayError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            _ => None,
        }
    }
}

impl From<ParseError> for CowsayError {
    fn from(err: ParseError) -> Self {
        CowsayError::CowfileParseError(err)
    }
}
