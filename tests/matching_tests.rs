#[cfg(test)]
mod tests {
    use afmt::config::*;

    #[test]
    fn source_target_tests() {
        for entry in std::fs::read_dir("tests/files").unwrap() {
            let entry = entry.unwrap();
            let source_path = entry.path();
            if source_path.extension().and_then(|ext| ext.to_str()) == Some("cls") {
                let file_path = source_path
                    .to_str()
                    .expect("PathBuf to String failed.")
                    .to_string();
                let session = Session::new(Config::default(), vec![file_path]);
                let vec = session.format();
                let output = vec
                    .into_iter()
                    .next()
                    .and_then(|result| result.ok())
                    .expect("format result failed.");

                let output_path = source_path.with_extension("out");

                let expected = std::fs::read_to_string(&output_path)
                    .expect(&format!("Failed to read output file: {:?}", output_path));

                assert_eq!(output, expected, "Mismatch in {}", source_path.display());
            }
        }
    }
}
