mod accessor;
pub mod args;
pub mod config;
mod data_model;
mod doc;
mod doc_builder;
mod enum_def;
mod macros;
mod utility;
//mod shape;
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
