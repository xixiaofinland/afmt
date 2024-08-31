use anyhow::{bail, Context, Result};
use tree_sitter::Node;

use crate::config::{Indent, Shape};

pub fn get_indent_string(indent: &Indent) -> String {
    let indent = "  ".repeat(indent.block_indent);
    indent
}

pub fn get_value<'a>(node: &Node, source_code: &'a str) -> &'a str {
    node.utf8_text(source_code.as_bytes())
        .expect(&format!("{}: get_value failed.", node.kind()))
}

pub fn get_mandatory_child_by_kind<'tree>(kind: &str, n: &Node<'tree>) -> Result<Node<'tree>> {
    get_child_by_kind(kind, n).ok_or(bail!(format!("{}: mandatory child not found.", kind)))
}

pub fn get_child_by_kind<'tree>(kind: &str, n: &Node<'tree>) -> Option<Node<'tree>> {
    let mut cursor = n.walk();
    let node = n.children(&mut cursor).find(|c| c.kind() == kind);
    node
}

pub fn get_mandatory_children_by_kind<'tree>(
    kind: &str,
    n: &Node<'tree>,
) -> Result<Vec<Node<'tree>>> {
    let children = get_children_by_kind(kind, n);
    if children.is_empty() {
        bail!("No children found with the kind: {}", kind);
    }

    Ok(children)
}

pub fn get_children_by_kind<'tree>(kind: &str, n: &Node<'tree>) -> Vec<Node<'tree>> {
    let mut cursor = n.walk();
    n.children(&mut cursor)
        .filter(|c| c.kind() == kind)
        .collect()
}

pub fn get_modifiers<'tree>(n: &Node<'tree>) -> Vec<Node<'tree>> {
    if let Some(node) = get_child_by_kind("modifiers", n) {
        get_children_by_kind("modifier", &node)
    } else {
        Vec::new()
    }
}

pub fn get_parameters<'tree>(n: &Node<'tree>) -> Vec<Node<'tree>> {
    if let Some(node) = n.child_by_field_name("parameters") {
        get_children_by_kind("formal_parameter", &node)
    } else {
        Vec::new()
    }
}

pub fn should_start_new_line<'tree>(node: &Node<'tree>) -> bool {
    if is_full_line_statement(node) {
        return true;
    }

    if let Some(parent) = node.parent() {
        match parent.kind() {
            "class_body" | "method_body" | "block" => {
                // Direct children of these blocks typically start on new lines
                // unless they're the opening or closing brace
                !matches!(node.kind(), "{" | "}")
            }
            "if_statement" => {
                // The condition doesn't start a new line, but the consequent and alternative do
                matches!(node.kind(), "block" | "if_statement") // For 'else if' and 'else' blocks
            }
            "for_statement" | "while_statement" => {
                // The body of loops typically starts on a new line
                node.kind() == "block"
            }
            "binary_expression" | "assignment_expression" => {
                // In long expressions, it's common to break after operators
                matches!(
                    node.kind(),
                    "+" | "-" | "*" | "/" | "=" | "==" | "!=" | "<" | ">" | "<=" | ">="
                )
            }
            // Add more parent-child relationships as needed
            _ => false,
        }
    } else {
        // Top-level nodes (direct children of the root) typically start on new lines
        true
    }
}

fn is_full_line_statement<'tree>(node: &Node<'tree>) -> bool {
    FULL_LINE_STATEMENTS.contains(&node.kind())
}

const FULL_LINE_STATEMENTS: &[&str] = &[
    "class_declaration",
    "method_declaration",
    "if_statement",
    "for_statement",
    "while_statement",
    "return_statement",
    "variable_declaration",
];
