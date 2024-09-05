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

pub fn get_mandatory_named_child_value<'a, 'tree>(
    name: &str,
    n: &Node<'tree>,
    source_code: &'a str,
) -> Result<&'a str> {
    let node = n
        .child_by_field_name(name)
        .context(format!("mandatory named field: {} missing.", name))?;
    Ok(get_value(&node, source_code))
}

pub fn get_mandatory_child_by_name<'tree>(name: &str, n: &Node<'tree>) -> Result<Node<'tree>> {
    n.child_by_field_name(name)
        .context(format!("mandatory named field: {} missing.", name))
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

pub fn get_modifiers_value<'tree>(n: &Node<'tree>, source_code: &str) -> String {
    let modifier_nodes = get_modifiers(n);
    modifier_nodes
        .iter()
        .map(|n| get_value(n, source_code))
        .collect::<Vec<&str>>()
        .join(" ")
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

pub fn is_standalone<'tree>(node: &Node<'tree>) -> bool {
    if !node.is_named() {
        return false;
    }

    if is_standalone_node(node) {
        return true;
    }

    if let Some(parent) = node.parent() {
        match parent.kind() {
            "class_body" | "block" => true,
            //"if_statement" => {
            //    matches!(node.kind(), "block" | "if_statement") // For 'else if' and 'else' blocks
            //}
            _ => false,
        }
    } else {
        !unreachable!() // all nodes should have a parent;
    }
}

pub fn is_standalone_node<'tree>(node: &Node<'tree>) -> bool {
    FULL_LINE_STATEMENTS.contains(&node.kind())
}

const FULL_LINE_STATEMENTS: &[&str] = &[
    "class_declaration",
    "method_declaration",
    "if_statement",
    "for_statement",
    "while_statement",
    "return_statement",
    "local_variable_declaration",
];

pub fn has_body_node<'tree>(node: &Node<'tree>) -> bool {
    HAS_BODY_NODE.contains(&node.kind())
}
const HAS_BODY_NODE: &[&str] = &[
    "class_declaration",
    "method_declaration",
    "if_statement",
    "for_statement",
    "while_statement",
];
