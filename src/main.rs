use anyhow::{bail, Context as AnyhowContext, Result};
use clap::{Arg, Command};
use context::{Context, CONTEXT};
use shape::Shape;
use std::{fs, path::Path};
use tree_sitter::{Node, Parser};
use utility::get_source_code;
use visitor::walk;

mod context;
mod extension;
mod node_struct;
mod shape;
mod utility;
mod visitor;

fn main() -> Result<()> {
    let mut parser = Parser::new();
    parser
        .set_language(&afmt::language())
        .expect("Error loading Apex grammar");

    let matches = Command::new("afmt")
        .version("1.0")
        .about("A CLI tool for formatting Apex code")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("The relative path to the file to parse")
                .default_value("samples/1.cls"),
        )
        .get_matches();

    let file_path = matches
        .get_one::<String>("file")
        .expect("File path is required");
    let path = Path::new(file_path);

    let source_code = fs::read_to_string(path).expect("Failed to read file");
    let source_code = Box::leak(source_code.into_boxed_str());
    let context = Context::new(source_code);
    CONTEXT.set(context).expect("Failed to set CONTEXT");

    let code_in_context = get_source_code();

    let tree = parser.parse(code_in_context, None).unwrap();
    let root_node = tree.root_node();

    if root_node.has_error() {
        bail!("parsing with error, bail out quickly.")
    }

    let result = format_code(&root_node).context("format_code() has `None` return.")?;
    println!("{}", result);
    Ok(())
}

fn format_code(root_node: &Node) -> Option<String> {
    let shape = Shape::default();
    walk(root_node, &shape)
}
