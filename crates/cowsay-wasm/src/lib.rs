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

#[derive(Debug, serde::Deserialize)]
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
pub struct OptionsConstructor {
    borg: Option<bool>,
    dead: Option<bool>,
    greedy: Option<bool>,
    sleepy: Option<bool>,
    tired: Option<bool>,
    wired: Option<bool>,
    young: Option<bool>,
    file: Option<String>,
    random: Option<bool>,
    eyes: Option<String>,
    tongue: Option<String>,
    wrap: Option<bool>,
    wrap_column: Option<usize>,
}

#[wasm_bindgen]
impl Options {
    #[wasm_bindgen(constructor)]
    pub fn new(options: JsValue) -> Result<Self, Error> {
        let options: OptionsConstructor =
            serde_wasm_bindgen::from_value(options).map_err(into_error)?;

        Ok(Self {
            borg: options.borg.unwrap_or(false),
            dead: options.dead.unwrap_or(false),
            greedy: options.greedy.unwrap_or(false),
            sleepy: options.sleepy.unwrap_or(false),
            tired: options.tired.unwrap_or(false),
            wired: options.wired.unwrap_or(false),
            young: options.young.unwrap_or(false),
            file: options.file,
            random: options.random.unwrap_or(false),
            eyes: options.eyes,
            tongue: options.tongue,
            wrap: options.wrap.unwrap_or(false),
            wrap_column: options.wrap_column,
        })
    }

    #[wasm_bindgen(constructor)]
    #[allow(clippy::missing_const_for_fn)]
    pub fn default_options() -> Self {
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

    fn to_cowsay_option(&self) -> CowsayOption<'_> {
        let mut builder = CowsayOption::builder()
            .with_borg(self.borg)
            .with_dead(self.dead)
            .with_greedy(self.greedy)
            .with_sleepy(self.sleepy)
            .with_tired(self.tired)
            .with_wired(self.wired)
            .with_young(self.young)
            .with_random(self.random)
            .with_wrap(self.wrap);

        if self.file.is_some() {
            let file = self.file.as_ref().unwrap();
            builder = builder.with_cowfile(file.as_str());
        }

        if self.eyes.is_some() {
            let eyes = self.eyes.as_ref().unwrap();
            builder = builder.with_eyes(eyes);
        }
        if self.tongue.is_some() {
            let tongue = self.tongue.as_ref().unwrap();
            builder = builder.with_tongue(tongue);
        }

        if self.wrap_column.is_some() {
            builder = builder
                .with_wrap(true)
                .with_wrap_column(self.wrap_column.unwrap());
        }

        builder.build()
    }

    pub fn say(&self, message: &str) -> Result<String, Error> {
        let cowsay_option = self.to_cowsay_option();
        let parser = cowsay_option.parser().map_err(into_error)?;

        if message.is_empty() {
            Error::new("Message cannot be empty");
        }

        let cow = parser.say(Some(message));
        Ok(cow)
    }
}
