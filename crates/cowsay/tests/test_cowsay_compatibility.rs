use std::fs::read_to_string;

use cowsay::{builder::CowsayOptionBuilder, CowsayOption};

const PHRASE: &str = "Hello, cowsay-rs!";

struct OptionTest {
    option: CowsayOptionBuilder,
    name: &'static str,
}

fn read_test_file(name: &str) -> String {
    let file_path = format!("tests/expected_outputs/{}.txt", name);
    read_to_string(file_path).expect("Failed to read test file")
}

#[test]
fn test_compatibility_with_options() {
    let options_test = vec![
        OptionTest {
            option: CowsayOption::builder().with_borg(true),
            name: "option_borg",
        },
        OptionTest {
            option: CowsayOption::builder().with_dead(true),
            name: "option_dead",
        },
        OptionTest {
            option: CowsayOption::builder().with_greedy(true),
            name: "option_greedy",
        },
        OptionTest {
            option: CowsayOption::builder().with_sleepy(true),
            name: "option_sleepy",
        },
        OptionTest {
            option: CowsayOption::builder().with_tired(true),
            name: "option_tired",
        },
        OptionTest {
            option: CowsayOption::builder().with_wired(true),
            name: "option_wired",
        },
        OptionTest {
            option: CowsayOption::builder().with_young(true),
            name: "option_young",
        },
    ];

    for option in options_test {
        let cowsay_option = option.option.build();
        let output = cowsay_option
            .parser()
            .expect("Failed to create parser")
            .say(Some(PHRASE));

        let expected_output = read_test_file(option.name);

        println!("Testing option: {}", option.name);
        println!("Output:\n{}", output);
        println!("Expected Output:\n{}", expected_output);
        assert_eq!(output, expected_output);
    }
}
