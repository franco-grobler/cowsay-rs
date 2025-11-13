use crate::CowsayOption;

/// A builder for creating `CowsayOption` instances.
///
/// This builder provides a fluent API to configure various aspects of the cowsay
/// message, such as the cow's appearance, the cow file to use, eyes, tongue,
/// and text wrapping options.
///
/// # Examples
///
/// ```
/// use cowsay::builder::CowsayOptionBuilder;
///
/// let options = CowsayOptionBuilder::default()
///     .with_message("Hello, world!")
///     .with_cowfile("default")
///     .with_eyes("oo")
///     .with_wrap_column(40)
///     .build();
/// ```
#[derive(Debug, Default)]
#[allow(clippy::struct_excessive_bools)]
pub struct CowsayOptionBuilder<'a> {
    borg: bool,
    dead: bool,
    greedy: bool,
    sleepy: bool,
    tired: bool,
    wired: bool,
    young: bool,
    file: Option<&'a str>,
    random: bool,
    eyes: Option<&'a str>,
    tongue: Option<&'a str>,
    wrap: bool,
    wrap_column: Option<usize>,
    message: Option<&'a str>,
}

impl<'a> CowsayOptionBuilder<'a> {
    /// Sets the message to be displayed by the cow.
    ///
    /// # Arguments
    ///
    /// * `message` - The text message for the cow to say.
    ///
    /// # Examples
    ///
    /// ```
    /// use cowsay::builder::CowsayOptionBuilder;
    ///
    /// let builder = CowsayOptionBuilder::default().with_message("Moo!");
    /// ```
    #[must_use]
    pub const fn with_message(mut self, message: &'a str) -> Self {
        self.message = Some(message);
        self
    }

    /// Configures the cow to have a "borg" appearance.
    ///
    /// # Arguments
    ///
    /// * `value` - `true` to enable borg appearance, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use cowsay::builder::CowsayOptionBuilder;
    ///
    /// let builder = CowsayOptionBuilder::default().with_borg(true);
    /// ```
    #[must_use]
    pub const fn with_borg(mut self, value: bool) -> Self {
        self.borg = value;
        self
    }

    /// Configures the cow to have a "dead" appearance.
    ///
    /// # Arguments
    ///
    /// * `value` - `true` to enable dead appearance, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use cowsay::builder::CowsayOptionBuilder;
    ///
    /// let builder = CowsayOptionBuilder::default().with_dead(true);
    /// ```
    #[must_use]
    pub const fn with_dead(mut self, value: bool) -> Self {
        self.dead = value;
        self
    }

    /// Configures the cow to have a "greedy" appearance.
    ///
    /// # Arguments
    ///
    /// * `value` - `true` to enable greedy appearance, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use cowsay::builder::CowsayOptionBuilder;
    ///
    /// let builder = CowsayOptionBuilder::default().with_greedy(true);
    /// ```
    #[must_use]
    pub const fn with_greedy(mut self, value: bool) -> Self {
        self.greedy = value;
        self
    }

    /// Configures the cow to have a "sleepy" appearance.
    ///
    /// # Arguments
    ///
    /// * `value` - `true` to enable sleepy appearance, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use cowsay::builder::CowsayOptionBuilder;
    ///
    /// let builder = CowsayOptionBuilder::default().with_sleepy(true);
    /// ```
    #[must_use]
    pub const fn with_sleepy(mut self, value: bool) -> Self {
        self.sleepy = value;
        self
    }

    /// Configures the cow to have a "tired" appearance.
    ///
    /// # Arguments
    ///
    /// * `value` - `true` to enable tired appearance, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use cowsay::builder::CowsayOptionBuilder;
    ///
    /// let builder = CowsayOptionBuilder::default().with_tired(true);
    /// ```
    #[must_use]
    pub const fn with_tired(mut self, value: bool) -> Self {
        self.tired = value;
        self
    }

    /// Configures the cow to have a "wired" appearance.
    ///
    /// # Arguments
    ///
    /// * `value` - `true` to enable wired appearance, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use cowsay::builder::CowsayOptionBuilder;
    ///
    /// let builder = CowsayOptionBuilder::default().with_wired(true);
    /// ```
    #[must_use]
    pub const fn with_wired(mut self, value: bool) -> Self {
        self.wired = value;
        self
    }

    /// Configures the cow to have a "young" appearance.
    ///
    /// # Arguments
    ///
    /// * `value` - `true` to enable young appearance, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use cowsay::builder::CowsayOptionBuilder;
    ///
    /// let builder = CowsayOptionBuilder::default().with_young(true);
    /// ```
    #[must_use]
    pub const fn with_young(mut self, value: bool) -> Self {
        self.young = value;
        self
    }

    /// Specifies the filename of the cowfile to use.
    ///
    /// # Arguments
    ///
    /// * `filename` - The name of the `.cow` file (without the extension).
    ///
    /// # Examples
    ///
    /// ```
    /// use cowsay::builder::CowsayOptionBuilder;
    ///
    /// let builder = CowsayOptionBuilder::default().with_cowfile("tux");
    /// ```
    #[must_use]
    pub const fn with_cowfile(mut self, filename: &'a str) -> Self {
        self.file = Some(filename);
        self
    }

    /// Configures the builder to select a random cowfile.
    ///
    /// # Arguments
    ///
    /// * `value` - `true` to enable random cowfile selection, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use cowsay::builder::CowsayOptionBuilder;
    ///
    /// let builder = CowsayOptionBuilder::default().with_random(true);
    /// ```
    #[must_use]
    pub const fn with_random(mut self, value: bool) -> Self {
        self.random = value;
        self
    }

    /// Sets the eyes string for the cow.
    ///
    /// # Arguments
    ///
    /// * `eyes` - A two-character string representing the cow's eyes (e.g., "oo", "==").
    ///
    /// # Examples
    ///
    /// ```
    /// use cowsay::builder::CowsayOptionBuilder;
    ///
    /// let builder = CowsayOptionBuilder::default().with_eyes("^^");
    /// ```
    #[must_use]
    pub const fn with_eyes(mut self, eyes: &'a str) -> Self {
        self.eyes = Some(eyes);
        self
    }

    /// Sets the tongue string for the cow.
    ///
    /// # Arguments
    ///
    /// * `tongue` - A two-character string representing the cow's tongue (e.g., "U ", "L ").
    ///
    /// # Examples
    ///
    /// ```
    /// use cowsay::builder::CowsayOptionBuilder;
    ///
    /// let builder = CowsayOptionBuilder::default().with_tongue("U ");
    /// ```
    #[must_use]
    pub const fn with_tongue(mut self, tongue: &'a str) -> Self {
        self.tongue = Some(tongue);
        self
    }

    /// Enables or disables text wrapping for the message.
    ///
    /// # Arguments
    ///
    /// * `value` - `true` to enable wrapping, `false` to disable.
    ///
    /// # Examples
    ///
    /// ```
    /// use cowsay::builder::CowsayOptionBuilder;
    ///
    /// let builder = CowsayOptionBuilder::default().with_wrap(true);
    /// ```
    #[must_use]
    pub const fn with_wrap(mut self, value: bool) -> Self {
        self.wrap = value;
        self
    }

    /// Sets the column at which the message should be wrapped.
    ///
    /// This option is only effective if `with_wrap(true)` is also set.
    ///
    /// # Arguments
    ///
    /// * `column` - The maximum column width for wrapping.
    ///
    /// # Examples
    ///
    /// ```
    /// use cowsay::builder::CowsayOptionBuilder;
    ///
    /// let builder = CowsayOptionBuilder::default().with_wrap_column(60);
    /// ```
    #[must_use]
    pub const fn with_wrap_column(mut self, column: usize) -> Self {
        self.wrap_column = Some(column);
        self
    }

    /// Builds a `CowsayOption` instance from the configured builder.
    ///
    /// # Returns
    ///
    /// A `CowsayOption` struct containing all the specified configurations.
    ///
    /// # Examples
    ///
    /// ```
    /// use cowsay::builder::CowsayOptionBuilder;
    ///
    /// let options = CowsayOptionBuilder::default()
    ///     .with_message("Hello")
    ///     .build();
    /// ```
    #[must_use]
    pub const fn build(self) -> CowsayOption<'a> {
        CowsayOption {
            borg: self.borg,
            dead: self.dead,
            greedy: self.greedy,
            sleepy: self.sleepy,
            tired: self.tired,
            wired: self.wired,
            young: self.young,
            file: self.file,
            random: self.random,
            eyes: self.eyes,
            tongue: self.tongue,
            wrap: self.wrap,
            wrap_column: self.wrap_column,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::CowsayOption;

    #[test]
    fn it_builds_default() {
        let options = CowsayOption::builder().build();
        assert!(!options.borg);
        assert!(!options.dead);
        assert!(!options.greedy);
        assert!(!options.sleepy);
        assert!(!options.tired);
        assert!(!options.wired);
        assert!(options.file.is_none());
        assert!(!options.random);
        assert!(options.eyes.is_none());
        assert!(options.tongue.is_none());
        assert!(!options.wrap);
        assert!(options.wrap_column.is_none());
    }

    #[test]
    fn it_builds_with_options() {
        let options = CowsayOption::builder()
            .with_borg(true)
            .with_dead(true)
            .with_greedy(true)
            .with_sleepy(true)
            .with_tired(true)
            .with_wired(true)
            .with_cowfile("custom.cow")
            .with_random(true)
            .with_eyes("&&")
            .with_tongue("U ")
            .with_wrap(true)
            .with_wrap_column(50)
            .with_message("Test message")
            .build();

        assert!(options.borg);
        assert!(options.dead);
        assert!(options.greedy);
        assert!(options.sleepy);
        assert!(options.tired);
        assert!(options.wired);
        assert_eq!(options.file.unwrap(), "custom.cow");
        assert!(options.random);
        assert_eq!(options.eyes.unwrap(), "&&");
        assert_eq!(options.tongue.unwrap(), "U ");
        assert!(options.wrap);
        assert_eq!(options.wrap_column.unwrap(), 50);
    }
}
