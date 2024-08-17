use afmt;
use std::fs;
use tree_sitter::{Node, Parser, Tree};

fn main() {
    let mut parser = Parser::new();
    parser
        .set_language(&afmt::language())
        .expect("Error loading Apex grammar");

    let code = fs::read_to_string("test/1.cls").unwrap();

    let tree = parser.parse(&code, None).unwrap();

    if tree.root_node().has_error() {
        println!("root node found error!");
        return;
    }

    let formatted_code = format_code(&tree, &code);
    println!("\n\nFormatted code:\n{}", formatted_code);
}

fn format_code(tree: &Tree, source_code: &str) -> String {
    let mut formatted = String::new();
    let mut indent_level = 0;

    format_node(
        tree.root_node(),
        source_code,
        &mut formatted,
        &mut indent_level,
    );

    formatted
}

fn format_node(node: Node, source_code: &str, formatted: &mut String, indent_level: &mut usize) {
    let node_type = node.kind();

    match node_type {
        "class_declaration" | "method_declaration" => {
            add_node_text(node, source_code, formatted);
            *indent_level += 1;
            formatted.push('\n');
        }
        "}" => {
            if *indent_level > 0 {
                *indent_level -= 1;
            }
            add_indent(formatted, *indent_level);
            formatted.push('}');
            formatted.push('\n');
        }
        "statement" => {
            add_indent(formatted, *indent_level);
            add_node_text(node, source_code, formatted);
            formatted.push('\n');
        }
        _ => {
            if node.child_count() == 0 {
                add_node_text(node, source_code, formatted);
            }
        }
    }

    for i in 0..node.child_count() {
        let child = node.child(i).unwrap();
        format_node(child, source_code, formatted, indent_level);
    }
}

fn add_node_text(node: Node, source_code: &str, formatted: &mut String) {
    let start_byte = node.start_byte();
    let end_byte = node.end_byte();
    formatted.push_str(&source_code[start_byte..end_byte]);
}

fn add_indent(formatted: &mut String, indent_level: usize) {
    for _ in 0..indent_level {
        formatted.push_str("    "); // 4 spaces per indent level
    }
}
