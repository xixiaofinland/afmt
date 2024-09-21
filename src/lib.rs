pub mod args;
mod child;
pub mod config;
mod context;
mod macros;
mod rewrite;
mod shape;
mod struct_and_enum;
mod utility;
mod visit;

use anyhow::Result;
use args::Args;
use config::{Config, Session};

pub fn format(args: Args) -> Vec<Result<String>> {
    let config = Config::default();
    let source_files = vec![args.path];
    let session = Session::new(config, source_files);
    session.format()
}
