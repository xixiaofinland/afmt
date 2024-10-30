mod accessor;
pub mod args;
pub mod config;
mod doc;
mod fmt_push;
mod macros;
mod print;
mod rewrite;
mod rich_def;
mod rich_macro;
mod rich_node;
mod rich_structs;
mod route;
mod shape;
mod struct_def;
mod utility;
mod visit;

use config::Session;

pub fn format(session: Session) {
    session.format();
}
