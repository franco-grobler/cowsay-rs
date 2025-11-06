use crate::errors::CowsayError;
use include_directory::{Dir, include_directory};

use rand::seq::IteratorRandom;

static COWS: Dir<'_> = include_directory!("$CARGO_MANIFEST_DIR/../../cows");

pub fn get_random_cow() -> String {
    let mut rng = rand::rng();
    let files = COWS
        .files()
        .map(|f| f.path().file_name().unwrap().to_string_lossy().to_string());
    let file = files.choose(&mut rng).unwrap();
    COWS.get_file(&file)
        .unwrap()
        .contents_utf8()
        .unwrap()
        .to_string()
}

pub fn get_cow_from_file(file_name: &str) -> Result<String, CowsayError> {
    let cowfile = COWS
        .get_file(file_name)
        .and_then(|f| f.contents_utf8().map(|s| s.to_string()));
    match cowfile.is_some() {
        true => Ok(cowfile.unwrap()),
        false => Err(CowsayError::CowfileNotFound(file_name.to_string())),
    }
}
