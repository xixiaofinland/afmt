pub mod args;
mod child;
pub mod config;
mod context;
mod macros;
mod rewrite;
mod route;
mod shape;
mod struct_def;
mod utility;
mod visit;

use anyhow::Result;
use config::Session;

pub fn format(session: Session) -> Vec<Result<String>> {
    session.format()
}
