mod accessor;
pub mod args;
pub mod config;
mod context;
mod fmt_push;
mod macros;
mod rewrite;
mod rich_def;
mod rich_macro;
mod rich_node;
mod route;
mod shape;
mod struct_def;
mod utility;
mod visit;

use config::Session;

pub fn format(session: Session) {
    session.format();
}
