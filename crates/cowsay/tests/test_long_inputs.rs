#[cfg(test)]
mod tests {

    #[test]
    fn test_long_input() {
        let long_input = "This is a very long input string that is intended to test how the cowsay implementation handles text wrapping and formatting when the input exceeds typical lengths. The quick brown fox jumps over the lazy dog. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";

        let cowsay_option =
            cowsay::CowsayOption::builder().with_wrap(true).build();
        let output = cowsay_option
            .parser()
            .expect("Failed to create parser")
            .say(Some(long_input));

        println!("{}", output);
        assert_eq!(
            output,
            r##"
 ________________________________________
/ "This is a very long input string that \
| is intended to test how the cowsay     |
| implementation handles text wrapping   |
| and formatting when the input exceeds  |
| typical lengths. The quick brown fox   |
| jumps over the lazy dog. Lorem ipsum   |
| dolor sit amet, consectetur adipiscing |
| elit. Sed do eiusmod tempor incididunt |
\ ut labore et dolore magna aliqua."     /
 ----------------------------------------
        \   ^__^
         \  (==)\_______
            (__)\       )\/\
                ||----w |
                ||     ||
"##
        )
    }
}
