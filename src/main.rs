use afmt::args::{get_args, Args};
use afmt::format;
use afmt::formatter::Formatter;
use std::time::Instant;
use std::{fs, process};

fn main() {
    let start = Instant::now();

    let result = run(get_args());

    match result {
        Ok(_) => {
            println!("Afmt completed successfully.");
            let duration = start.elapsed();
            println!("\nExecution time: {:?}", duration);
            process::exit(0);
        }
        Err(e) => {
            eprint!("Error: {}", e);
            process::exit(1);
        }
    }
}

fn run(args: Args) -> Result<(), String> {
    let formatter = Formatter::create_from_config(args.config.as_deref(), vec![args.path.clone()])?;
    let results = format(formatter);

    for (index, result) in results.iter().enumerate() {
        match result {
            Ok(value) => {
                if args.write {
                    fs::write(&args.path, value).map_err(|e| {
                        format!("Failed to write formatted content to {}: {}", args.path, e)
                    })?;
                    println!("Formatted content written back to: {}\n", args.path);
                } else {
                    println!("Result {}: Ok\n{}", index, value);
                }
            }
            Err(e) => {
                //println!("Result {}: Err\n{}", index, e);
                return Err(format!("Error processing result {}: {}", index, e));
            }
        }
    }

    Ok(())
}
