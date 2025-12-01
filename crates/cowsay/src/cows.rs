use crate::errors::CowsayError;
use cowsay_template::errors::ParseError;
use include_directory::{Dir, include_directory};

use rand::rng;
use rand::seq::IteratorRandom;

static COWS: Dir<'_> = include_directory!("$CARGO_MANIFEST_DIR/../../cows");

/// Selects a random cowfile from the embedded cowfiles.
///
/// This function reads the list of available cowfiles and randomly selects one.
/// The content of the selected cowfile is then returned as a `String`.
///
/// # Returns
///
/// A `Result` which is:
/// - `Ok(String)` containing the content of a randomly selected cowfile.
/// - `Err(CowsayError::CowfileNotFound)` if no cowfiles are found.
/// - `Err(CowsayError::CowfileParseError)` if the selected cowfile cannot be
///   read or is not valid UTF-8.
///
/// # Panics
///
/// This function panics if `file_name().unwrap()` is called on a `None` value,
/// which can happen if a directory entry does not have a valid file name.
/// This is highly unlikely with the embedded cowfiles, but theoretically possible
/// if the `include_directory!` macro includes malformed entries.
///
/// # Examples
///
/// ```
/// use cowsay::cows;
///
/// let random_cow_template = cows::get_random_cow().unwrap();
/// println!("{}", random_cow_template);
/// ```
pub fn get_random_cow() -> Result<String, CowsayError> {
    let mut rng = rng();
    let file_name = COWS
        .files()
        .map(|f| f.path().file_name().unwrap().to_string_lossy().to_string())
        .choose(&mut rng)
        .ok_or_else(|| {
            CowsayError::CowfileNotFound("random cow".to_string())
        })?;
    get_cow_from_file(&file_name)
}

/// Reads the content of a specified cowfile from the embedded cowfiles.
///
/// This function attempts to find and read the content of a cowfile
/// identified by `file_name`. The content is returned as a `String`.
///
/// # Arguments
///
/// * `file_name` - The name of the cowfile to retrieve (e.g., "default.cow").
///
/// # Returns
///
/// A `Result` which is:
/// - `Ok(String)` containing the content of the specified cowfile.
/// - `Err(CowsayError::CowfileNotFound)` if the specified cowfile does not exist.
/// - `Err(CowsayError::CowfileParseError)` if the cowfile cannot be read
///   or is not valid UTF-8.
///
/// # Examples
///
/// ```
/// use cowsay::cows;
///
/// let default_cow_template = cows::get_cow_from_file("default.cow").unwrap();
/// println!("{}", default_cow_template);
/// ```
pub fn get_cow_from_file(file_name: &str) -> Result<String, CowsayError> {
    let file = COWS.get_file(file_name);
    file.map_or_else(
        || Err(CowsayError::CowfileNotFound(file_name.to_string())),
        |f| {
            f.contents_utf8().map_or_else(
                || {
                    Err(CowsayError::CowfileParseError(
                        ParseError::InvalidTemplateFormat(format!(
                            "cowfile '{file_name}' is not valid UTF-8"
                        )),
                    ))
                },
                |contents| Ok(contents.to_string()),
            )
        },
    )
}

/// Returns a list of all cows included by default.
pub fn list_cows() -> Vec<String> {
    COWS.files()
        .filter_map(|f| f.path().as_os_str().to_str())
        .filter_map(|c| c.strip_suffix(".cow"))
        .map(Into::into)
        .collect()
}
