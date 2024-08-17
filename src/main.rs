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
    format_code(&tree, &code);
    //let formatted_code = format_code(&tree, &code);
    //println!("\n\nResult:\n{}", formatted_code);
}

fn format_code(tree: &Tree, source_code: &str) -> String {
    if tree.root_node().has_error() {
        println!("root node found error!");
        return String::new();
    }

    let mut formatted = String::new();
    let mut indent_level = 0;

    let mut cursor = tree.walk();
    if cursor.goto_first_child() {
        loop {
            let node = cursor.node();

            match node.kind() {
                "class_declaration" => {
                    format_class_node(node, source_code, &mut indent_level);
                }

                _ => {
                    unimplemented!()
                }
            }

            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }

    formatted
}

fn format_class_node(node: Node, source_code: &str, indent_level: &mut usize) {
    println!("node kind: {}", node.kind());
    let mut formatted = String::new();

    //match node.kind() {
    //    "class_declaration" | "method_declaration" => {
    //        add_indent(formatted, *indent_level);
    //        add_node_text(node, source_code, formatted);
    //        formatted.push(' ');
    //    }
    //    "{" => {
    //        formatted.push_str("{\n");
    //        *indent_level += 1;
    //    }
    //    "}" => {
    //        if *indent_level > 0 {
    //            *indent_level -= 1;
    //        }
    //        add_indent(formatted, *indent_level);
    //        formatted.push_str("}\n");
    //    }
    //    "statement" => {
    //        add_indent(formatted, *indent_level);
    //        add_node_text(node, source_code, formatted);
    //        if !formatted.ends_with(';') {
    //            formatted.push(';');
    //        }
    //        formatted.push('\n');
    //    }
    //    _ => {
    //        if node.child_count() == 0 {
    //            let text = node_text(node, source_code);
    //            if !text.trim().is_empty() {
    //                formatted.push_str(text);
    //                if !node.kind().ends_with("_keyword") && !text.ends_with(' ') {
    //                    formatted.push(' '); // Add space after non-keyword tokens
    //                }
    //            }
    //        }
    //    }
    //}
    println!("\n---");
    println!("{}", formatted);
    println!("---\n");
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

fn map(node: Node, child_name: &str) -> Vec<&str> {
    let mut result = Vec::new();
    for i in 0..node.named_child_count() {
        let child = node.named_child(i).unwrap();
        if child.kind() == child_name {
            result.push(child_name);
        }
    }
    result
}
