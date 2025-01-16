use clap::{Arg as ClapArg, Command};

#[derive(Debug)]
pub struct Args {
    pub path: String,
    pub config: Option<String>,
    pub write: bool,
}

pub fn get_args() -> Args {
    const VERSION: &str = "v0.1.2";

    let matches = Command::new("afmt")
        .version(VERSION)
        .about(format!("Apex format tool (afmt): {}", VERSION))
        .arg_required_else_help(true)
        .arg(
            ClapArg::new("file")
                .value_name("FILE")
                .help("The relative path to the file to parse")
                .required(true)
                .index(1),
        )
        .arg(
            ClapArg::new("config")
                .short('c')
                .long("config")
                .value_name("CONFIG")
                .help("Path to the .afmt.toml configuration file"),
        )
        .arg(
            ClapArg::new("write")
                .short('w')
                .long("write")
                .help("Write the formatted result back to the file")
                .action(clap::ArgAction::SetTrue),
        )
        .after_help(
            "EXAMPLES:\n\
             \n\
             # Dry run: print the result without overwriting the file\n\
             afmt ./file.cls\n\
             \n\
             # Format and write changes back to the file\n\
             afmt --write src/file.cls\n\
             \n\
             # Use a specific config file\n\
             afmt --config .afmt.toml ./file.cls\n\
            ",
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
