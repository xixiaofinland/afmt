use sf_afmt::args::{get_args, Args};
use sf_afmt::format;
use sf_afmt::formatter::Formatter;
use std::time::Instant;
use std::{fs, process};

fn main() {
    let start = Instant::now();

    let args = get_args();
    let result = run(&args);

    match result {
        Ok(_) => {
            if args.time {
                let duration = start.elapsed();
                println!("\n-- Execution time: {:?}", duration);
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
    let formatter = Formatter::create_from_config(args.config.as_deref(), vec![args.path.clone()])?;
    let results = format(formatter);

    for (index, result) in results.iter().enumerate() {
        match result {
            Ok(value) => {
                if args.check {
                    let original_content = fs::read_to_string(&args.path)
                        .map_err(|e| format!("Failed to read file {}: {}", args.path, e))?;

                    if original_content.as_str() == value {
                        println!("File is already formatted: {}", args.path);
                        return Ok(());
                    } else {
                        eprintln!("File is not correctly formatted: {}", args.path);
                        return Err("Formatting check failed".to_string());
                    }
                } else if args.write {
                    fs::write(&args.path, value).map_err(|e| {
                        format!("Failed to write formatted content to {}: {}", args.path, e)
                    })?;
                    println!("Formatted content written back to: {}\n", args.path);
                } else {
                    //println!("Result {}: Ok\n{}", index, value);
                    println!("{}", value);
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
