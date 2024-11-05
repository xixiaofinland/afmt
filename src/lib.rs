mod accessor;
pub mod args;
mod data_model;
mod doc;
mod doc_builder;
mod enum_def;
pub mod formatter;
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

use formatter::Formatter;

pub fn format(session: Formatter) {
    session.format();
}
