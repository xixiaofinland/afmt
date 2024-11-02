mod accessor;
pub mod args;
pub mod config;
mod doc;
mod enum_def;
mod macros;
mod node_to_doc;
mod print;
mod shape;
mod struct_def;
mod utility;
//mod rewrite;
//mod rich_macro;
//mod rich_node;
//mod rich_structs;
//mod route;
//mod fmt_push;
//mod visit;

use config::Session;

pub fn format(session: Session) {
    session.format();
}
