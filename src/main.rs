use anyhow::{bail, Result};
use clap::{Arg, Command};
use context::Context;
use shape::Shape;
use std::{fs, path::Path};
use tree_sitter::{Node, Parser};
use visitor::Visitor;

mod context;
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

    let code = fs::read_to_string(path).expect("Failed to read file");
    let tree = parser.parse(&code, None).unwrap();
    let root_node = tree.root_node();

    if root_node.has_error() {
        bail!("parsing with error, bail out quickly.")
    }

    let result = format_code(&root_node, &code)?;
    println!("\n####\n\n---\n{}\n---", result);
    Ok(())
}

fn format_code(root_node: &Node, source_code: &str) -> Result<String> {
    let context = Context::new(source_code);
    let mut visitor = Visitor::new(context);
    let shape = Shape::default();
    visitor.walk(root_node, shape)
}

fn add_node_text(node: Node, source_code: &str, formatted: &mut String) {
    formatted.push_str(node_text(node, source_code));
}

fn node_text<'a>(node: Node, source_code: &'a str) -> &'a str {
    let start_byte = node.start_byte();
    let end_byte = node.end_byte();
    println!("node text: {}", &source_code[start_byte..end_byte]);
    &source_code[start_byte..end_byte]
}

fn add_indent(formatted: &mut String, indent_level: usize) {
    for _ in 0..indent_level {
        formatted.push_str("    "); // 4 spaces per indent level
    }
}

fn call(node: Node, child_name: &str) -> String {
    if let Some(child) = node.child_by_field_name(child_name) {
        return "parsed!".into();
    }

    String::new()
}
