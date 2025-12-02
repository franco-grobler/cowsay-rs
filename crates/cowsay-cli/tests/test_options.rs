//! Run integration tests for different cow options.

#[cfg(test)]
mod test {
    use assert_cmd::cargo::cargo_bin_cmd;
    use predicates::str::contains;
    use pretty_assertions::assert_str_eq;

    #[test]
    fn test_borg() {
        let expected_output = [
            r" _______________",
            r"< Hello, World! >",
            r" ---------------",
            r"        \   ^__^",
            r"         \  (==)\_______",
            r"            (__)\       )\/\",
            r"                ||----w |",
            r"                ||     ||",
            "",
        ]
        .join("\n");

        let mut cmd = cargo_bin_cmd!("cowsay-rs");
        cmd.arg("-b").arg("Hello, World!");
        cmd.assert().success();
        assert_str_eq!(
            String::from_utf8(cmd.output().unwrap().stdout).unwrap(),
            expected_output
        );
    }

    #[test]
    fn test_dead() {
        let expected_output = [
            r" _______________",
            r"< Hello, World! >",
            r" ---------------",
            r"        \   ^__^",
            r"         \  (xx)\_______",
            r"            (__)\       )\/\",
            r"             U  ||----w |",
            r"                ||     ||",
            "",
        ]
        .join("\n");

        let mut cmd = cargo_bin_cmd!("cowsay-rs");
        cmd.arg("-d").arg("Hello, World!");
        cmd.assert().success();
        assert_str_eq!(
            String::from_utf8(cmd.output().unwrap().stdout).unwrap(),
            expected_output
        );
    }

    #[test]
    fn test_gready() {
        let expected_output = [
            r" _______________",
            r"< Hello, World! >",
            r" ---------------",
            r"        \   ^__^",
            r"         \  ($$)\_______",
            r"            (__)\       )\/\",
            r"                ||----w |",
            r"                ||     ||",
            "",
        ]
        .join("\n");

        let mut cmd = cargo_bin_cmd!("cowsay-rs");
        cmd.arg("-g").arg("Hello, World!");
        cmd.assert().success();
        assert_str_eq!(
            String::from_utf8(cmd.output().unwrap().stdout).unwrap(),
            expected_output
        );
    }
    #[test]
    fn test_sleepy() {
        let expected_output = [
            r" _______________",
            r"< Hello, World! >",
            r" ---------------",
            r"        \   ^__^",
            r"         \  (**)\_______",
            r"            (__)\       )\/\",
            r"             U  ||----w |",
            r"                ||     ||",
            "",
        ]
        .join("\n");

        let mut cmd = cargo_bin_cmd!("cowsay-rs");
        cmd.arg("-s").arg("Hello, World!");
        cmd.assert().success();
        assert_str_eq!(
            String::from_utf8(cmd.output().unwrap().stdout).unwrap(),
            expected_output
        );
    }
    #[test]
    fn test_tired() {
        let expected_output = [
            r" _______________",
            r"< Hello, World! >",
            r" ---------------",
            r"        \   ^__^",
            r"         \  (--)\_______",
            r"            (__)\       )\/\",
            r"                ||----w |",
            r"                ||     ||",
            "",
        ]
        .join("\n");

        let mut cmd = cargo_bin_cmd!("cowsay-rs");
        cmd.arg("-t").arg("Hello, World!");
        cmd.assert().success();
        assert_str_eq!(
            String::from_utf8(cmd.output().unwrap().stdout).unwrap(),
            expected_output
        );
    }

    #[test]
    fn test_young() {
        let expected_output = [
            r" _______________",
            r"< Hello, World! >",
            r" ---------------",
            r"        \   ^__^",
            r"         \  (..)\_______",
            r"            (__)\       )\/\",
            r"                ||----w |",
            r"                ||     ||",
            "",
        ]
        .join("\n");

        let mut cmd = cargo_bin_cmd!("cowsay-rs");
        cmd.arg("-y").arg("Hello, World!");
        cmd.assert().success();
        assert_str_eq!(
            String::from_utf8(cmd.output().unwrap().stdout).unwrap(),
            expected_output
        );
    }

    #[test]
    fn test_wired() {
        let expected_output = [
            r" _______________",
            r"< Hello, World! >",
            r" ---------------",
            r"        \   ^__^",
            r"         \  (OO)\_______",
            r"            (__)\       )\/\",
            r"                ||----w |",
            r"                ||     ||",
            "",
        ]
        .join("\n");

        let mut cmd = cargo_bin_cmd!("cowsay-rs");
        cmd.arg("-w").arg("Hello, World!");
        cmd.assert().success();
        assert_str_eq!(
            String::from_utf8(cmd.output().unwrap().stdout).unwrap(),
            expected_output
        );
    }

    #[test]
    fn test_random() {
        let mut cmd = cargo_bin_cmd!("cowsay-rs");
        cmd.arg("-r").arg("Hello, World!");
        cmd.assert().success().stdout(contains(
            [
                r" _______________",
                r"< Hello, World! >",
                r" ---------------",
            ]
            .join("\n"),
        ));
    }

    #[test]
    fn test_configure_looks() {
        let expected_output = [
            r" ________",
            r"/ Hello, \",
            r"\ World! /",
            r" --------",
            r"        \   ^__^",
            r"         \  (EE)\_______",
            r"            (__)\       )\/\",
            r"             T  ||----w |",
            r"                ||     ||",
            "",
        ]
        .join("\n");

        let mut cmd = cargo_bin_cmd!("cowsay-rs");
        cmd.arg("-e=EE")
            .arg("-T=T ")
            .arg("-W=7")
            .arg("Hello, World!");
        cmd.assert().success();
        assert_str_eq!(
            String::from_utf8(cmd.output().unwrap().stdout).unwrap(),
            expected_output
        );
    }
}
