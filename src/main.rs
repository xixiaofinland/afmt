use afmt::args::{get_args, Args};
use afmt::format;
use anyhow::Result;
use colored::Colorize;
use log::error;
use log::info;
use std::panic;
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;

//fn main() {
//    let start = Instant::now();
//    env_logger::init();
//    info!("starting up");
//
//    if let Err(e) = run(get_args()) {
//        eprintln!("{e}");
//        std::process::exit(1);
//    }
//    let duration = start.elapsed();
//    println!("\n{} {:?}", "Execution time:".green(), duration);
//}

fn main() {
    let start = Instant::now();
    env_logger::init();
    info!("starting up");

    let panic_occurred = Arc::new(AtomicBool::new(false));
    let panic_occurred_clone = Arc::clone(&panic_occurred);

    // Set up panic hook
    panic::set_hook(Box::new(move |panic_info| {
        panic_occurred_clone.store(true, Ordering::SeqCst);
        if let Some(location) = panic_info.location() {
            error!(
                "Panic occurred in file '{}' at line {}",
                location.file(),
                location.line(),
            );
        } else {
            error!("Panic occurred but can't get location information...");
        }
        error!("Panic info: {:?}", panic_info);
    }));

    // Run the main logic
    let result = run(get_args());

    // Check if a panic occurred
    if panic_occurred.load(Ordering::SeqCst) {
        error!("Program panicked");
        process::exit(2);
    }

    // Handle the result
    match result {
        Ok(_) => {
            println!("Program completed successfully.");
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
    let results = format(args);
    for (index, result) in results.iter().enumerate() {
        match result {
            Ok(value) => {
                println!("Result {}: Ok\n{}", index, value);
                println!("{}", value.replace('\n', "\\n"));
            }
            Err(e) => {
                println!("Result {}: Err\n{}", index, e);
                return Err(anyhow::anyhow!("{}", e));
            }
        }
    }
    Ok(())
}
