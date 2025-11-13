use crate::CowsayOption;

#[derive(Debug, Default)]
/// Builder for `CowsayOption`.
///
/// * `borg`: Borg appearance.
/// * `dead`: Dead appearance.
/// * `greedy`: Greedy appearance.
/// * `sleepy`: Sleepy appearance.
/// * `tired`: Tired appearance.
/// * `wired`: Wired appearance.
/// * `young`: Young appearance.
/// * `file`: Filename of the cowfile to use.
/// * `random`: Choose a random cowfile.
/// * `eyes`: Eyes string.
/// * `tongue`: Tongue string.
/// * `wrap`: Text wrapping.
/// * `wrap_column`: Wrap column number.
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
}

impl<'a> CowsayOptionBuilder<'a> {
    /// Sets borg appearance.
    ///
    /// * `value`: Should borg appearance be used.
    pub const fn with_borg(mut self, value: bool) -> Self {
        self.borg = value;
        self
    }

    /// Sets dead appearance.
    ///
    /// * `value`: Should dead appearance be used.
    pub const fn with_dead(mut self, value: bool) -> Self {
        self.dead = value;
        self
    }

    /// Sets greedy appearance.
    ///
    /// * `value`: Should greedy appearance be used.
    pub const fn with_greedy(mut self, value: bool) -> Self {
        self.greedy = value;
        self
    }

    /// Sets sleepy appearance.
    ///
    /// * `value`: Should sleepy appearance be used.
    pub const fn with_sleepy(mut self, value: bool) -> Self {
        self.sleepy = value;
        self
    }

    /// Sets tired appearance.
    ///
    /// * `value`: Should tired appearance be used.
    pub const fn with_tired(mut self, value: bool) -> Self {
        self.tired = value;
        self
    }

    /// Sets wired appearance.
    ///
    /// * `value`: Should wired appearance be used.
    pub const fn with_wired(mut self, value: bool) -> Self {
        self.wired = value;
        self
    }

    /// Sets young appearance.
    ///
    /// * `value`: Use young appearance.
    pub const fn with_young(mut self, value: bool) -> Self {
        self.young = value;
        self
    }

    /// Set cowfile name.
    ///
    /// * `filename`: Name of the cowfile to use.
    pub const fn with_file(mut self, filename: &'a str) -> Self {
        self.file = Some(filename);
        self
    }

    /// Set random cowfile usage.
    ///
    /// * `value`: Should a random cowfile be used.
    pub const fn with_random(mut self, value: bool) -> Self {
        self.random = value;
        self
    }

    /// Set the eyes string.
    ///
    /// * `eyes`: String to use for the eyes.
    pub const fn with_eyes(mut self, eyes: &'a str) -> Self {
        self.eyes = Some(eyes);
        self
    }

    /// Set the tongue string.
    ///
    /// * `tongue`: String to use for the tongue.
    pub const fn with_tongue(mut self, tongue: &'a str) -> Self {
        self.tongue = Some(tongue);
        self
    }

    /// Set wrapping behavior.
    ///
    /// * `value`: Should wrapping be enabled.
    pub const fn with_wrap(mut self, value: bool) -> Self {
        self.wrap = value;
        self
    }

    /// Sets the wrap column.
    ///
    /// * `column`: Column number to wrap at.
    pub const fn with_wrap_column(mut self, column: usize) -> Self {
        self.wrap_column = Some(column);
        self
    }

    /// Creates a `CowsayOption` from the builder.
    pub fn build(self) -> CowsayOption<'a> {
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
            .with_file("custom.cow")
            .with_random(true)
            .with_eyes("&&")
            .with_tongue("U ")
            .with_wrap(true)
            .with_wrap_column(50)
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
