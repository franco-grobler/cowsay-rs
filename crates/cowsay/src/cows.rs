use crate::errors::CowsayError;
use cowsay_template::errors::ParseError;
use include_directory::{Dir, include_directory};

use rand::seq::IteratorRandom;

static COWS: Dir<'_> = include_directory!("$CARGO_MANIFEST_DIR/../../cows");

pub fn get_random_cow() -> Result<String, CowsayError> {
    let mut rng = rand::rng();
    let file_name = COWS
        .files()
        .map(|f| f.path().file_name().unwrap().to_string_lossy().to_string())
        .choose(&mut rng)
        .ok_or_else(|| {
            CowsayError::CowfileNotFound("random cow".to_string())
        })?;
    get_cow_from_file(&file_name)
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
