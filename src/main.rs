use sf_afmt::args::{get_args, Args};
use sf_afmt::format;
use sf_afmt::formatter::{Formatter, Mode};
use std::io::{self, IsTerminal, Write};
use std::process;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let result = run(get_args());

    match result {
        Ok(_) => {
            if std::io::stdout().is_terminal() {
                println!("Afmt completed successfully.");
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

fn run(args: Args) -> Result<(), String> {
    let formatter =
        Formatter::create_from_config(args.config.as_deref(), args.mode.to_owned(), args.paths)?;
    let results = format(formatter);
    let mut stdout = io::stdout();
    let is_terminal = stdout.is_terminal();

    for (index, result) in results.iter().enumerate() {
        match (result, args.mode.to_owned()) {
            (Ok(value), Mode::Std) => {
                stdout.write_all(value.as_bytes()).unwrap();
            }
            (Ok(value), Mode::Check) => {
                let message = if is_terminal {
                    format!("Result {}: Ok", index)
                } else {
                    index.to_string()
                };
                println!("{}", message);
            }
            (Ok(value), Mode::Write) => {
                println!("Result {}: Ok", index);
            }
            (Err(e), _) => {
                return Err(format!("Error processing result {}: {}", index, e));
            }
        }
    }

    Ok(())
}
