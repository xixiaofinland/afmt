#[cfg(test)]
mod tests {
    use afmt::format_code;
    use std::path::PathBuf;

    #[test]
    fn source_target_tests() {
        for entry in std::fs::read_dir("tests/source").unwrap() {
            let entry = entry.unwrap();
            let source_path = entry.path();

            if source_path.extension().and_then(|ext| ext.to_str()) == Some("cls") {
                let source_code = std::fs::read_to_string(&source_path).unwrap();
                let output = format_code(&source_code).unwrap();

                let target_path = PathBuf::from("tests/target")
                    .join(source_path.file_stem().unwrap())
                    .with_extension("stdout");

                let expected = std::fs::read_to_string(&target_path).unwrap();

                assert_eq!(output, expected, "Mismatch in {}", source_path.display());
            }
        }
    }

    fn run_afmt(file_path: &std::path::Path) -> String {
        let output = std::process::Command::new("cargo")
            .args(["run", "--", "-f"])
            .arg(file_path)
            .output()
            .expect("failed to execute cargo run");
        String::from_utf8(output.stdout).unwrap()
    }
}
