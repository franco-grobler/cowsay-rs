//! Tests for cowsay with long input strings to verify proper text wrapping and formatting.

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_str_eq;

    #[test]
    fn test_long_input() {
        let long_input = "This is a very long input string that is intended to test how the cowsay implementation handles text wrapping and formatting when the input exceeds typical lengths. The quick brown fox jumps over the lazy dog. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";

        let cowsay_option =
            cowsay::CowsayOption::builder().with_wrap(true).build();
        let output = cowsay_option
            .parser()
            .expect("Failed to create parser")
            .say(Some(long_input));
        let expected_output = [
            r" ________________________________________",
            r"/ This is a very long input string that  \",
            r"| is intended to test how the cowsay     |",
            r"| implementation handles text wrapping   |",
            r"| and formatting when the input exceeds  |",
            r"| typical lengths. The quick brown fox   |",
            r"| jumps over the lazy dog. Lorem ipsum   |",
            r"| dolor sit amet, consectetur adipiscing |",
            r"| elit. Sed do eiusmod tempor incididunt |",
            r"\ ut labore et dolore magna aliqua.      /",
            r" ----------------------------------------",
            r"        \   ^__^",
            r"         \  (oo)\_______",
            r"            (__)\       )\/\",
            r"                ||----w |",
            r"                ||     ||",
            "",
        ]
        .join("\n");

        assert_str_eq!(output, expected_output);
    }
}
