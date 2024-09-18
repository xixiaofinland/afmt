#[cfg(test)]
mod tests {
    use afmt::config::*;
    use similar::{ChangeTag, TextDiff};
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::{Path, PathBuf};
    use std::process::Command;

    #[test]
    fn source_target_tests() {
        for entry in std::fs::read_dir("tests/prettier").unwrap() {
            let entry = entry.unwrap();
            let in_file = entry.path();
            if in_file.extension().and_then(|ext| ext.to_str()) == Some("in") {
                println!("### Processing file: {:?}", in_file);
                run_test_files(&in_file);
            }
        }
    }

    fn run_test_files(in_file: &Path) {
        let prettier_file = in_file.with_extension("pre");

        if !prettier_file.exists() {
            println!("### .pre file not found, generating...");
            let prettier_output = run_prettier(in_file).expect("Failed to run Prettier");
            save_prettier_output(&prettier_file, &prettier_output);
        }

        let output = format_with_afmt(in_file);
        let prettier_output =
            std::fs::read_to_string(&prettier_file).expect("Failed to read the .pre file.");

        if output != prettier_output {
            let file_content =
                std::fs::read_to_string(&in_file).expect("Failed to read the file content.");

            println!("\nFailed: {:?}:", in_file);
            println!("-------------------------------------\n");
            println!("{}", file_content);
            println!("-------------------------------------\n");
            print_side_by_side_diff(&output, &prettier_output);
            println!("\n-------------------------------------\n");

            assert_eq!(output, prettier_output, "Mismatch in {}", in_file.display());
        }
    }

    fn format_with_afmt(source_path: &Path) -> String {
        let file_path = source_path
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

    fn save_prettier_output(pre_file_path: &PathBuf, content: &str) {
        let mut file = File::create(pre_file_path).expect("Unable to create .pre file");
        file.write_all(content.as_bytes())
            .expect("Unable to write to .pre file");
    }

    fn run_prettier(source_path: &Path) -> Result<String, String> {
        let output = Command::new("npx")
            .arg("prettier")
            .arg("--plugin=prettier-plugin-apex")
            .arg("--parser=apex")
            .arg(source_path.to_str().unwrap())
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

    fn print_side_by_side_diff(output: &str, expected: &str) {
        let diff = TextDiff::from_lines(expected, output);
        let mut left_col = String::new();
        let mut right_col = String::new();

        println!(
            "\x1b[38;2;255;165;0m{:<40} | {:<40}\x1b[0m",
            "Prettier:", "Afmt:\n"
        );

        for change in diff.iter_all_changes() {
            match change.tag() {
                ChangeTag::Delete => {
                    left_col = format!("\x1b[91m- {:<38}\x1b[0m", change.to_string().trim_end()); // Red for deletions (left)
                    right_col = String::from(""); // Empty on the right side
                }
                ChangeTag::Insert => {
                    left_col = String::from(""); // Empty on the left side
                    right_col = format!("\x1b[92m+ {:<38}\x1b[0m", change.to_string().trim_end());
                    // Green for insertions (right)
                }
                ChangeTag::Equal => {
                    left_col = format!("  {:<38}", change.to_string().trim_end()); // No color for unchanged lines
                    right_col = format!("  {:<38}", change.to_string().trim_end());
                }
            }

            // Print the two columns side-by-side
            println!("{:<40} | {:<40}", left_col, right_col);
        }
    }
}
