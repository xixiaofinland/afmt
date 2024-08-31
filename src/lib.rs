pub mod args;
mod config;
mod context;
mod macros;
mod node_struct;
mod utility;
mod visitor;

use anyhow::Result;
use args::Args;
use config::{Config, Session};

pub fn format(args: Args) -> Vec<Result<String>> {
    let config = Config::default();
    let source_files = vec![args.path];
    let session = Session::new(config, source_files);
    session.format()
}
