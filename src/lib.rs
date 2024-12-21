mod accessor;
pub mod args;
mod data_model;
mod doc;
mod doc_builder;
mod enum_def;
pub mod formatter;
mod utility;
use anyhow::Result;

mod wasm_entry;
pub use wasm_entry::*;

use formatter::Formatter;

#[cfg(not(target_arch = "wasm32"))]
pub fn format(f: Formatter) -> Vec<Result<String>> {
    f.format()

    //let result = f.format();
    //match &result[0] {
    //    Ok(data) => println!("\n###\n{}\n###\n", data),
    //    Err(_) => eprintln!("error in result"),
    //}
}
