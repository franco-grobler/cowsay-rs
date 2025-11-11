use crate::errors::CowsayError;
use cowsay_template::errors::ParseError;
use include_directory::{Dir, include_directory};

use rand::seq::IteratorRandom;

static COWS: Dir<'_> = include_directory!("$CARGO_MANIFEST_DIR/../../cows");

pub fn get_random_cow() -> Result<String, CowsayError> {
    let mut rng = rand::rng();
    let files = COWS
        .files()
        .map(|f| f.path().file_name().unwrap().to_string_lossy().to_string());
    let file_name = files.choose(&mut rng).unwrap();
    let file = COWS
        .get_file(&file_name)
        .map(|f| f.contents_utf8().map(|s| s.to_string()));
    match file.is_some() {
        true => Ok(file.unwrap().unwrap()),
        false => Err(CowsayError::CowfileNotFound(file_name.to_string())),
    }
}

pub fn get_cow_from_file(file_name: &str) -> Result<String, CowsayError> {
    let file = COWS.get_file(file_name);
    match file {
        Some(f) => match f.contents_utf8() {
            Some(contents) => Ok(contents.to_string()),
            None => Err(CowsayError::CowfileParseError(
                ParseError::InvalidTemplateFormat(format!(
                    "cowfile '{}' is not valid UTF-8",
                    file_name
                )),
            )),
        },
        None => Err(CowsayError::CowfileNotFound(file_name.to_string())),
    }
}
