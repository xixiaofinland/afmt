use afmt;
use anyhow::{bail, Result};
use std::fs;
use tree_sitter::{Node, Parser, Tree};
use visitor::Visitor;

mod node_struct;
mod utility;
mod visitor;

fn main() -> Result<()> {
    let mut parser = Parser::new();
    parser
        .set_language(&afmt::language())
        .expect("Error loading Apex grammar");
    let code = fs::read_to_string("test/1.cls").unwrap();
    let tree = parser.parse(&code, None).unwrap();
    let root_node = tree.root_node();

    if root_node.has_error() {
        bail!("parsing with error, bail out quickly.")
    }

    let result = format_code(&root_node)?;
    println!("format result: {}", result);
    Ok(())
}

fn format_code(root_node: &Node) -> Result<String> {
    let mut visitor = Visitor::default();
    visitor.walk(&root_node)
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
