mod accessor;
pub mod args;
mod data_model;
mod doc;
mod doc_builder;
mod enum_def;
pub mod formatter;
mod utility;

use formatter::Formatter;

pub fn format(f: Formatter) {
    let result = f.format();
    match &result[0] {
        Ok(data) => println!("\n###\n{}\n###\n", data),
        Err(_) => eprintln!("error in result"),
    }
}
