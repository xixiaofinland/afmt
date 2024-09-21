#[cfg(test)]
mod tests {
    use afmt::config::*;
    use colored::Colorize;
    use similar::{ChangeTag, TextDiff};
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    use std::process::Command;

    #[test]
    fn manual() {
        for entry in std::fs::read_dir("tests/static").unwrap() {
            let entry = entry.unwrap();
            let source = entry.path();
            if source.extension().and_then(|ext| ext.to_str()) == Some("in") {
                println!("{} {:?}", "### Processing static file:".green(), source);
                run_static_test_files(&source);
            }
        }
    }

    #[test]
    fn prettier() {
        println!("Running Prettier-based tests...");
        for entry in std::fs::read_dir("tests/prettier").unwrap() {
            let entry = entry.unwrap();
            let source = entry.path();
            if source.extension().and_then(|ext| ext.to_str()) == Some("in") {
                println!("{} {:?}", "### Processing prettier file:".green(), source);
                run_prettier_test_files(&source);
            }
        }
    }

    fn run_static_test_files(source: &Path) {
        let expected_file = source.with_extension("cls");
        let output = format_with_afmt(source);
        let expected =
            std::fs::read_to_string(expected_file).expect("Failed to read expected .cls file");

        compare("Static:", output, expected, source);
    }

    fn run_prettier_test_files(source: &Path) {
        let prettier_file = source.with_extension("pre");

        if !prettier_file.exists() {
            println!("{}", "### .pre file not found, generating...".yellow());
            let prettier_output = run_prettier(source).expect("Failed to run Prettier");
            save_prettier_output(&prettier_file, &prettier_output);
        }

        let output = format_with_afmt(source);
        let prettier_output =
            std::fs::read_to_string(&prettier_file).expect("Failed to read the .pre file.");

        compare("Prettier:", output, prettier_output, source);
    }

    fn compare(against: &str, output: String, expected: String, source: &Path) {
        if output != expected {
            let source_content =
                std::fs::read_to_string(source).expect("Failed to read the file content.");

            println!("\nFailed: {:?}:", source);
            println!("-------------------------------------\n");
            println!("{}", source_content);
            println!("-------------------------------------\n");
            print_side_by_side_diff(against, &output, &expected);
            println!("\n-------------------------------------\n");

            assert_eq!(expected, output, "Mismatch in {}", source.display());
        }
    }

    fn format_with_afmt(source: &Path) -> String {
        let file_path = source
            .to_str()
            .expect("PathBuf to String failed.")
            .to_string();
        let session = Session::new(Config::default(), vec![file_path.clone()]);
        let vec = session.format();
        vec.into_iter()
            .next()
            .and_then(|result| result.ok())
            .expect("format result failed.")
    }

    fn print_side_by_side_diff(against: &str, output: &str, expected: &str) {
        let diff = TextDiff::from_lines(expected, output);
        let mut left_col = String::new();
        let mut right_col = String::new();
        println!(
            "\x1b[38;2;255;165;0m{:<60} | {:<60}\x1b[0m",
            against, "Afmt:\n"
        );
        for change in diff.iter_all_changes() {
            match change.tag() {
                ChangeTag::Delete => {
                    left_col = format!("\x1b[91m- {:<58}\x1b[0m", change.to_string().trim_end()); // Red for deletions (left)
                    right_col = String::from(""); // Empty on the right side
                }
                ChangeTag::Insert => {
                    left_col = String::from(""); // Empty on the left side
                    right_col = format!("\x1b[92m+ {:<58}\x1b[0m", change.to_string().trim_end());
                    // Green for insertions (right)
                }
                ChangeTag::Equal => {
                    left_col = format!("  {:<58}", change.to_string().trim_end()); // No color for unchanged lines
                    right_col = format!("  {:<58}", change.to_string().trim_end());
                }
            }
            println!("{:<60} | {:<60}", left_col, right_col);
        }
    }

    fn run_prettier(source: &Path) -> Result<String, String> {
        let output = Command::new("npx")
            .arg("prettier")
            .arg("--plugin=prettier-plugin-apex")
            .arg("--parser=apex")
            .arg(source.to_str().unwrap())
            .output()
            .expect("Failed to execute Prettier");

        if output.status.success() {
            let formatted_code =
                String::from_utf8(output.stdout).expect("Prettier output is not valid UTF-8");
            Ok(formatted_code)
        } else {
            let error_message = String::from_utf8(output.stderr)
                .unwrap_or_else(|_| "Unknown error while running Prettier".to_string());
            Err(error_message)
        }
    }

    fn save_prettier_output(file_path: &Path, output: &str) {
        let mut file = File::create(file_path).expect("Failed to create .pre file");
        file.write_all(output.as_bytes())
            .expect("Failed to write Prettier output");
    }
}
