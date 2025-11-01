#[derive(Debug)]
struct Cow {
    eyes: String,
    tongue: String,
    file: String,
    text: String,
    thoughts: String,
    thinking: bool,
    balloonWidth: i8,
    wordWrap: bool,
}

impl Default for Cow {
    fn default() -> Self {
        Cow {
            eyes: "oo".to_string(),
            tongue: "  ".to_string(),
            file: "default.cow".to_string(),
            text: "".to_string(),
            thoughts: "o".to_string(),
            thinking: false,
            balloonWidth: 40,
            wordWrap: true,
        }
    }
}
