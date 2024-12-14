mod accessor;
pub mod args;
mod data_model;
mod doc;
mod doc_builder;
mod enum_def;
pub mod formatter;
mod utility;
use anyhow::Result;

use formatter::Formatter;

pub fn format(f: Formatter) -> Vec<Result<String>> {
    f.format()

    //let result = f.format();
    //match &result[0] {
    //    Ok(data) => println!("\n###\n{}\n###\n", data),
    //    Err(_) => eprintln!("error in result"),
    //}
}
