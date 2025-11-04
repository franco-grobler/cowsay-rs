#[cfg(test)]
mod tests {
    use glob::glob;
    use std::fs::read_to_string;

    use cowsay::{builder::CowsayOptionBuilder, CowsayOption};

    const PHRASE: &str = "Hello, cowsay-rs!";

    struct OptionTest {
        option: CowsayOptionBuilder,
        name: String,
    }

    fn read_test_file(name: String) -> String {
        let file_path = format!("tests/expected_outputs/{}.txt", name);
        read_to_string(file_path).expect("Failed to read test file")
    }

    #[test]
    fn test_compatibility_with_options() {
        let options_test = vec![
            OptionTest {
                option: CowsayOption::builder().with_borg(true),
                name: "option_borg".to_string(),
            },
            OptionTest {
                option: CowsayOption::builder().with_dead(true),
                name: "option_dead".to_string(),
            },
            OptionTest {
                option: CowsayOption::builder().with_greedy(true),
                name: "option_greedy".to_string(),
            },
            OptionTest {
                option: CowsayOption::builder().with_sleepy(true),
                name: "option_sleepy".to_string(),
            },
            OptionTest {
                option: CowsayOption::builder().with_tired(true),
                name: "option_tired".to_string(),
            },
            OptionTest {
                option: CowsayOption::builder().with_wired(true),
                name: "option_wired".to_string(),
            },
            OptionTest {
                option: CowsayOption::builder().with_young(true),
                name: "option_young".to_string(),
            },
        ];

        for option in options_test {
            let cowsay_option = option.option.build();
            let output = cowsay_option
                .parser()
                .expect("Failed to create parser")
                .say(Some(PHRASE));

            let expected_output = read_test_file(option.name);

            assert_eq!(output, expected_output);
        }
    }

    #[test]
    fn test_compatibility_with_files() {
        let mut options_test: Vec<OptionTest> = vec![];
        for entry in glob("tests/expected_outputs/file_*.txt")
            .expect("Failed to read glob pattern")
        {
            let file_name = entry
                .as_ref()
                .unwrap()
                .file_prefix()
                .unwrap()
                .to_string_lossy();
            let cow_name = file_name
                .as_ref()
                .strip_prefix("file_")
                .unwrap()
                .to_string();

            options_test.push(OptionTest {
                option: CowsayOption::builder().with_file(cow_name),
                name: file_name.to_string(),
            });
        }

        for option in options_test {
            let cowsay_option = option.option.build();
            let output = cowsay_option
                .parser()
                .expect("Failed to create parser")
                .say(Some(PHRASE));

            let expected_output = read_test_file(option.name);

            println!("Output:\n{}", output);
            println!("Expected Output:\n{}", expected_output);
            assert_eq!(output, expected_output);
        }
    }
}
