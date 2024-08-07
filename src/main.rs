use std::fs;
use tree_sitter::{Language, Node, Parser};

extern "C" {
    fn tree_sitter_apex() -> Language;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the parser
    let mut parser = Parser::new();
    let language = unsafe { tree_sitter_apex() };
    parser.set_language(&language)?;

    // Read the input file
    let code = fs::read_to_string("input.cls")?;

    // Parse the code
    let tree = parser.parse(&code, None).unwrap();
    let root_node = tree.root_node();

    // TODO: Implement formatting logic here
    println!("AST structure:");
    print_node(&root_node, 0);

    // TODO: Output formatted code

    Ok(())
}

fn print_node(node: &Node, depth: usize) {
    let indent = "  ".repeat(depth);
    println!("{}{}:", indent, node.kind());

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        print_node(&child, depth + 1);
    }
}
