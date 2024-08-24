#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    #[test]
    fn source_target_tests() {
        for entry in std::fs::read_dir("tests/source").unwrap() {
            let entry = entry.unwrap();
            let source_path = entry.path();

            if source_path.extension().and_then(|ext| ext.to_str()) == Some("cls") {
                let output = run_afmt(&source_path);

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
