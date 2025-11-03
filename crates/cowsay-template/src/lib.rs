use std::path::Path;

#[derive(Debug)]
pub struct CowTemplate<'a> {
    path: &'a Path,
    variables: Vec<String>,
}

#[derive(Debug)]
pub struct CowTemplateResult {
    pub rendered: String,
    pub description: String,
}

impl<'a> CowTemplate<'a> {
    pub fn new(path: &'a Path, variables: Vec<String>) -> Self {
        CowTemplate { path, variables }
    }

    pub fn render(&self) -> String {
        // Placeholder implementation
        format!(
            "Rendering template from {:?} with variables: {:?}",
            self.path, self.variables
        )
    }

    pub fn render_with_description(&self) -> CowTemplateResult {
        CowTemplateResult {
            rendered: self.render(),
            description: String::from("Rendered cow template"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Write};

    use super::*;
    use tempfile::tempdir;

    #[test]
    fn it_works_with_template() {
        let temp_dir = tempdir().unwrap();
        let test_cow = temp_dir.path().join("test.cow");
        let cow_file = File::create(&test_cow).unwrap();
        let lines = [
            "$the_cow = <<\"EOC\";",
            "        $thoughts   ^__^",
            r"         $thoughts  ($eyes)\_______",
            r"            (__)\       )\/\",
            r"             $tongue ||----w |",
            r"                ||     ||",
            "EOC",
        ];
        for line in lines {
            writeln!(&cow_file, "{}", line).unwrap();
        }

        let cow = CowTemplate::new(
            test_cow.as_path(),
            vec![String::from(r"\"), String::from("oo"), String::from("")],
        );
        let output = cow.render();
        let expected_output = [
            r"        \   ^__^",
            r"         \  (oo)\_______",
            r"            (__)\       )\/\",
            r"                ||----w |",
            r"                ||     ||",
        ]
        .join("\n");

        println!("Output:\n{}", output);
        assert_eq!(expected_output, output);
        drop(cow_file);
        temp_dir.close().unwrap();
    }
}
