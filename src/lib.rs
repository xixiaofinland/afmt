pub mod args;
mod child;
pub mod config;
mod context;
mod enrich_def;
mod enrich_macro;
mod enrich_nodes;
mod fmt_push;
mod macros;
mod rewrite;
mod route;
mod shape;
mod struct_def;
mod utility;
mod visit;

use config::Session;

pub fn format(session: Session) {
    session.format();
}
