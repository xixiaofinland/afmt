use std::time::Instant;

use afmt::args::{get_args, Args};
use afmt::format;
use anyhow::Result;
use colored::Colorize;
use log::info;

fn main() {
    let start = Instant::now();
    env_logger::init();
    info!("starting up");

    if let Err(e) = run(get_args()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
    let duration = start.elapsed();
    println!("\n{} {:?}", "Execution time:".green(), duration);
}

fn run(args: Args) -> Result<()> {
    let results = format(args);
    for (index, result) in results.iter().enumerate() {
        match result {
            Ok(value) => {
                println!("Result {}: Ok\n{}", index, value);
                println!("{}", value.replace('\n', "\\n"));
            }
            Err(e) => println!("Result {}: Err\n{}", index, e),
        }
    }
    Ok(())
}
