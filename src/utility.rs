use anyhow::{Context, Result};
use tree_sitter::Node;

use crate::config::{Indent, Shape};

pub fn get_indent_string(indent: &Indent) -> String {
    let indent = "  ".repeat(indent.block_indent);
    indent
}

pub fn get_value<'a>(node: &Node, source_code: &'a str) -> Result<&'a str> {
    node.utf8_text(source_code.as_bytes())
        .context("get node source code failed.")
}

pub fn get_child_by_kind<'tree>(kind: &str, n: &Node<'tree>) -> Option<Node<'tree>> {
    let mut cursor = n.walk();
    let node = n.children(&mut cursor).find(|c| c.kind() == kind);
    node
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
