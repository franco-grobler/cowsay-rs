//! A WebAssembly wrapper for the `cowsay` crate, enabling its use in JavaScript
//! environments.
//!
//! This crate provides a `cowsay` function that can be called from JavaScript
//! to generate ASCII art of a cow saying a message. It exposes an `Options`
//! struct to allow customization of the cow's appearance and behavior.
//!
//! # Examples
//!
//! ```javascript
//! import { Options } from 'cowsay-wasm';
//!
//! const options = new Options({
//!   message: 'Hello from WASM!',
//!   cow: 'default',
//!   wrap: true,
//! });
//!
//! const cowMessage = options.say();
//! console.log(cowMessage);
//! ```

use cowsay::CowsayOption;
use js_sys::Error;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

pub(crate) fn into_error<E: std::fmt::Display>(err: E) -> Error {
    Error::new(&err.to_string())
}

/// Defines the configuration options for a `cowsay` message in a WebAssembly context.
///
/// This struct holds all the customizable parameters for generating a cowsay
/// message, such as the cow's appearance (e.g., `borg`, `dead`), the cowfile
/// to use, eye and tongue strings, and text wrapping settings.
///
/// Instances of `Options` are created using the `new` constructor or
/// `default_options` for a default configuration.
#[derive(Debug)]
#[allow(clippy::struct_excessive_bools)]
#[wasm_bindgen]
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

/// A helper struct for deserializing `Options` from a JavaScript object.
///
/// This struct is used internally by the `Options::new` constructor to
/// deserialize a `JsValue` into a structured format. All fields are optional
/// to allow for partial configuration from the JavaScript side.
#[derive(Debug, serde::Deserialize)]
#[allow(clippy::struct_excessive_bools)]
#[wasm_bindgen]
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
    /// Creates a new `Options` instance from a JavaScript object.
    ///
    /// This constructor accepts a `JsValue` representing a JavaScript object
    /// with configuration properties. Missing properties will be set to their
    /// default values.
    ///
    /// # Arguments
    ///
    /// * `options` - A `JsValue` containing the configuration options.
    ///
    /// # Returns
    ///
    /// A `Result` which is:
    /// - `Ok(Options)` if the `JsValue` is successfully parsed.
    /// - `Err(Error)` if the `JsValue` cannot be deserialized into `OptionsConstructor`.
    ///
    /// # Examples
    ///
    /// ```javascript
    /// import { Options } from 'cowsay-wasm';
    ///
    /// const options = new Options({
    ///   file: 'tux',
    ///   eyes: '^^',
    /// });
    /// ```
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

    /// Creates a new `Options` instance with default values.
    ///
    /// This provides a convenient way to get a default set of options, which
    /// can then be used to generate a standard cowsay message.
    ///
    /// # Returns
    ///
    /// An `Options` struct with default settings (e.g., `wrap` is `true`,
    /// all appearance modes are `false`).
    ///
    /// # Examples
    ///
    /// ```javascript
    /// import { Options } from 'cowsay-wasm';
    ///
    /// const defaultOptions = Options.default_options();
    /// ```
    #[wasm_bindgen]
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

    /// Generates the cowsay message based on the configured options.
    ///
    /// This method takes the current `Options` and a message string, then
    /// invokes the core `cowsay` logic to produce the final ASCII art.
    ///
    /// # Arguments
    ///
    /// * `message` - The text message for the cow to say.
    ///
    /// # Returns
    ///
    /// A `Result` which is:
    /// - `Ok(String)` containing the complete cowsay message.
    /// - `Err(Error)` if there is an issue with parsing the options or
    ///   generating the message (e.g., cowfile not found).
    ///
    /// # Examples
    ///
    /// ```javascript
    /// import { Options } from 'cowsay-wasm';
    ///
    /// const options = Options.default_options();
    /// const cowMessage = options.say('Moo!');
    /// console.log(cowMessage);
    /// ```
    pub fn say(&self, message: &str) -> Result<String, Error> {
        let cowsay_option = self.to_cowsay_option();
        let parser = cowsay_option.parser().map_err(into_error)?;

        if message.is_empty() {
            return Err(Error::new("Message cannot be empty"));
        }

        let cow = parser.say(Some(message));
        Ok(cow)
    }
}
