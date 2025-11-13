//! WebAssembly version of cowsay.

use cowsay::CowsayOption;
use js_sys::Error;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

pub(crate) fn into_error<E: std::fmt::Display>(err: E) -> Error {
    Error::new(&err.to_string())
}

#[derive(Debug)]
#[allow(clippy::struct_excessive_bools)]
#[wasm_bindgen]
/// Options cowsay accepts.
///
/// * `borg`: Borg appearance.
/// * `dead`: Dead appearance.
/// * `greedy`: Greedy appearance.
/// * `sleepy`: Sleepy appearance.
/// * `tired`: Tired appearance.
/// * `wired`: Wired appearance.
/// * `young`: Young appearance.
/// * `file`: Template file name, or template string.
/// * `random`: Use a random template.
/// * `eyes`: Characters to use for the eyes.
/// * `tongue`: Characters to use for the tongue.
/// * `wrap`: Should the text be wrapped?
/// * `wrap_column`: Column width to wrap text at.
pub struct Options {
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

#[wasm_bindgen]
impl Options {
    #[wasm_bindgen(constructor)]
    pub fn new(options: JsValue) -> Result<Self, Error> {
        let options: Options =
            serde_wasm_bindgen::from_value(options).map_err(into_error)?;

        Ok(Self {
            borg: options.borg,
            dead: options.dead,
            greedy: options.greedy,
            sleepy: options.sleepy,
            tired: options.tired,
            wired: options.wired,
            young: options.young,
            file: options.file,
            random: options.random,
            eyes: options.eyes,
            tongue: options.tongue,
            wrap: options.wrap,
            wrap_column: options.wrap_column,
        })
    }

    #[wasm_bindgen(constructor)]
    pub fn default() -> Self {
        Self {
            borg: false,
            dead: false,
            greedy: false,
            sleepy: false,
            tired: false,
            wired: false,
            young: false,
            file: None,
            random: false,
            eyes: None,
            tongue: None,
            wrap: true,
            wrap_column: None,
        }
    }

    fn to_cowsay_option(&self) -> CowsayOption {
        CowsayOption::builder()
            .with_borg(self.borg)
            .with_dead(self.dead)
            .with_greedy(self.greedy)
            .with_sleepy(self.sleepy)
            .with_tired(self.tired)
            .with_wired(self.wired)
            .with_young(self.young)
            .with_file(self.file.clone())
            .with_random(self.random)
            .with_eyes(self.eyes.clone())
            .with_tongue(self.tongue.clone())
            .with_wrap(self.wrap)
            .with_wrap_column(self.wrap_column)
            .build()
    }

    pub fn say(&self, message: &str) -> Result<String, Error> {
        let cowsay_option = self.to_cowsay_option();
        let parser = match cowsay_option.parser() {
            Ok(p) => p,
            Err(e) => into_error(e),
        };

        if message.is_empty() {
            Err(into_error("Message cannot be empty"))
        }

        let cow = parser.say(message);
        Ok(cow)
    }
}
