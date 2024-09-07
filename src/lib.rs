pub mod args;
pub mod config;
mod context;
mod format;
mod macros;
mod node_ext;
mod node_struct;
mod shape;
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
