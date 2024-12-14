use clap::{Arg as ClapArg, Command};

#[derive(Debug)]
pub struct Args {
    pub path: String,
    pub config: Option<String>,
    pub write: bool,
}

pub fn get_args() -> Args {
    const VERSION: &str = "v0.0.16";

    let matches = Command::new("afmt")
        .version(VERSION)
        .about(format!("Format Apex file {}", VERSION))
        .arg(
            ClapArg::new("file")
                .value_name("FILE")
                .help("The relative path to the file to parse")
                .default_value("test.cls")
                .index(1),
        )
        .arg(
            ClapArg::new("config")
                .short('c')
                .long("config")
                .value_name("CONFIG")
                .help("Path to the .afmt.toml configuration file")
                .default_value("./.afmt.toml"),
        )
        .arg(
            ClapArg::new("write")
                .short('w')
                .long("write")
                .help("Write the formatted result back to the file")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    Args {
        path: matches
            .get_one::<String>("file")
            .expect("File path is required")
            .to_string(),
        config: matches.get_one::<String>("config").map(|s| s.to_string()),
        write: matches.get_flag("write"),
    }
}
