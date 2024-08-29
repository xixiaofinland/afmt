use afmt::config::Args;
use afmt::format;
use anyhow::Result;
use clap::{Arg as ClapArg, Command};

fn main() {
    if let Err(e) = run(get_args()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn get_args() -> Args {
    let matches = Command::new("afmt")
        .version("1.0")
        .about("A CLI tool for formatting Apex code")
        .arg(
            ClapArg::new("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("The relative path to the file to parse")
                .default_value("samples/1.cls"),
        )
        .get_matches();

    Args {
        path: matches
            .get_one::<String>("file")
            .expect("File path is required")
            .to_string(),
    }
}

fn run(args: Args) -> Result<()> {
    format(args);
    Ok(())
}
