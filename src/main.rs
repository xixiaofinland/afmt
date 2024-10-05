use afmt::args::{get_args, Args};
use afmt::config::{Config, Session};
use afmt::format;
use anyhow::{anyhow, Result};
use colored::Colorize;
use log::error;
use log::info;
use std::process;
use std::time::Instant;

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
    // Try to load the configuration file
    let config = Config::from_file(&args.config).unwrap_or_else(|_| {
        println!("Config file not found or invalid, using default configuration.");
        Config::default()
    });

    let session = Session::new(config, vec![args.path]);
    let results = format(session);

    for (index, result) in results.iter().enumerate() {
        match result {
            Ok(value) => {
                println!("Result {}: Ok\n{}", index, value);
                println!("{:?}", value)
            }
            Err(e) => {
                println!("Result {}: Err\n{}", index, e);
                return Err(anyhow!("{}", e));
            }
        }
    }
    Ok(())
}
