use clap::{Arg as ClapArg, Command};

#[derive(Debug)]
pub struct Args {
    pub path: String,
    pub config: String,
}

pub fn get_args() -> Args {
    let matches = Command::new("afmt")
        .version("0.1")
        .about("Format Apex file")
        .arg(
            ClapArg::new("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("The relative path to the file to parse")
                .default_value("tests/prettier/1.cls"),
        )
        .arg(
            ClapArg::new("config")
                .short('c')
                .long("config")
                .value_name("CONFIG")
                .help("Path to the .afmt.toml configuration file")
                .default_value(".afmt.toml"),
        )
        .get_matches();

    Args {
        path: matches
            .get_one::<String>("file")
            .expect("File path is required")
            .to_string(),
        config: matches
            .get_one::<String>("config")
            .expect("Config path is required")
            .to_string(),
    }
}
