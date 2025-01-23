use sf_afmt::args::{get_args, Args};
use sf_afmt::format;
use sf_afmt::formatter::{Formatter, Mode};
use sf_afmt::message_helper::yellow;
use std::io::{self, IsTerminal, Write};
use std::process;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let args = get_args();
    match run(&args) {
        Ok(_) => {
            if std::io::stdout().is_terminal() {
                println!(
                    "\nAfmt completed successfully. Processed {} files",
                    args.paths.len().max(1)
                );
                let duration = start.elapsed();
                println!("\nExecution time: {:?}", duration);
            }
            process::exit(0);
        }
        Err(e) => {
            eprint!("Error: {}", e);
            process::exit(1);
        }
    }
}

fn run(args: &Args) -> Result<(), String> {
    let formatter = Formatter::create_from_config(
        args.config.as_deref(),
        args.mode.to_owned(),
        args.paths.to_owned(),
    )?;
    let results = format(formatter);
    let mut stdout = io::stdout();
    let is_terminal = stdout.is_terminal();

    for (index, format_result) in results.iter().enumerate() {
        let format_result = format_result
            .as_ref()
            .map_err(|e| format!("Error processing file {}: {}", index, e))?;
        let file_path = format_result.file_path.to_str().unwrap_or_default();

        match (args.mode.to_owned(), format_result.is_changed, is_terminal) {
            (Mode::Std, _, _) => {
                stdout
                    .write_all(format_result.formatted_code.as_bytes())
                    .map_err(|e| format!("Couldn't write formatted code to stdout: {}", e))?;
            }
            (_, true, false) => {
                println!("{}", file_path);
            }
            (Mode::Write, true, true) => {
                println!("file {} has been formatted", file_path);
            }
            (Mode::Check, true, true) => {
                println!(
                    "{}",
                    yellow(&format!("file {} needs formatting", file_path))
                );
            }
            (_, false, true) => {
                println!("file {} is already formatted", file_path);
            }
            _ => {}
        }
    }

    Ok(())
}
