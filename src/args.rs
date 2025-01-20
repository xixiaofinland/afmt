use crate::formatter::Mode;
use clap::{Arg as ClapArg, ArgGroup, Command, Id, ValueHint};
use std::{io::IsTerminal, path::PathBuf};

#[derive(Debug)]
pub struct Args {
    pub paths: Vec<PathBuf>,
    pub config: Option<String>,
    pub mode: Mode,
}

pub fn get_args() -> Args {
    let version = env!("CARGO_PKG_VERSION"); // read from Cargo.toml in compiling time

    let mut command = Command::new("afmt")
        .version(version)
        .about(format!("Apex format tool (afmt): {}", version))
        .arg(
            ClapArg::new("config")
                .long("config")
                .value_name("CONFIG")
                .help("Path to the .afmt.toml configuration file")
                .value_hint(ValueHint::FilePath),
        )
        .arg(
            ClapArg::new("check")
                .short('c')
                .long("check")
                .num_args(1..)
                .value_parser(clap::value_parser!(PathBuf))
                .help("Check files for formatting issues")
                .value_hint(ValueHint::FilePath),
        )
        .arg(
            ClapArg::new("write")
                .short('w')
                .long("write")
                .num_args(1..)
                .value_parser(clap::value_parser!(PathBuf))
                .help("Write the formatted result back to the files")
                .value_hint(ValueHint::FilePath),
        )
        .group(ArgGroup::new("check_or_write").arg("check").arg("write"))
        .after_help(
            "EXAMPLES:\n\
             \n\
             # Read source code from stdin and output to stdout without writing any files\n\
             afmt < ./file.cls\n\
             \n\
             # Check if any of the specified files would be modified\n\
             afmt --check ./file.cls\n\
             \n\
             # Format and write changes back to the file\n\
             afmt --write src/file.cls\n\
             \n\
             # Use a specific config file\n\
             afmt --config .afmt.toml ./file.cls\n\
            ",
        );

    let matches = command.get_matches_mut();

    let mode = match matches.get_one::<Id>("check_or_write").map(|i| i.as_str()) {
        Some("check") => Mode::Check,
        Some("write") => Mode::Write,
        _ => Mode::Std,
    };

    if Mode::Std == mode && std::io::stdin().is_terminal() {
        command.print_help().unwrap();
        std::process::exit(2);
    }

    let paths = match mode {
        Mode::Check => matches
            .get_many::<PathBuf>("check")
            .into_iter()
            .flatten()
            .cloned()
            .collect(),
        Mode::Write => matches
            .get_many::<PathBuf>("write")
            .into_iter()
            .flatten()
            .cloned()
            .collect(),
        Mode::Std => vec![],
    };

    Args {
        config: matches.get_one::<String>("config").map(|s| s.to_string()),
        mode,
        paths,
    }
}
