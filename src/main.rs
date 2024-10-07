use afmt::args::{get_args, Args};
use afmt::config::{Config, Session};
use afmt::format;
use anyhow::{anyhow, Result};
use colored::Colorize;
use log::error;
use log::info;
use std::time::Instant;
use std::{fs, process};

fn main() {
    let start = Instant::now();
    env_logger::init();
    info!("starting up");

    let result = run(get_args());

    match result {
        Ok(_) => {
            println!("Afmt completed successfully.");
            let duration = start.elapsed();
            println!("\n{} {:?}", "Execution time:".green(), duration);
            process::exit(0);
        }
        Err(e) => {
            error!("Error: {}", e);
            process::exit(1);
        }
    }
}

fn run(args: Args) -> Result<()> {
    eprintln!("gopro[5]: main.rs:32: args={:#?}", args);
    let session =
        Session::create_session_from_config(args.config.as_deref(), vec![args.path.clone()])?;
    let results = format(session);

    for (index, result) in results.iter().enumerate() {
        match result {
            Ok(value) => {
                println!("Result {}: Ok\n{}", index, value);
                if args.write {
                    fs::write(&args.path, value)?;
                    println!("Formatted content written back to: {}\n", args.path);
                } else {
                    println!("{:?}", value)
                }
            }
            Err(e) => {
                println!("Result {}: Err\n{}", index, e);
                return Err(anyhow!("{}", e));
            }
        }
    }
    Ok(())
}
