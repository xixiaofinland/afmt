use crate::{context::FmtContext, node_ext::*, shape::Shape};
use anyhow::{bail, Context, Result};
use tree_sitter::Node;

use crate::shape::Indent;

pub fn get_indent_string(indent: &Indent) -> String {
    let indent = "  ".repeat(indent.block_indent);
    indent
}

pub fn get_modifiers_value<'tree>(n: &Node<'tree>, source_code: &str) -> String {
    let modifier_nodes = get_modifiers(n);
    modifier_nodes
        .iter()
        .map(|n| n.get_value(source_code))
        .collect::<Vec<&str>>()
        .join(" ")
}

pub fn get_modifiers<'tree>(n: &Node<'tree>) -> Vec<Node<'tree>> {
    if let Some(c) = n.get_child_by_kind("modifiers") {
        c.get_children_by_kind("modifier")
    } else {
        Vec::new()
    }
}

pub fn get_parameters<'tree>(n: &Node<'tree>) -> Vec<Node<'tree>> {
    if let Some(c) = n.child_by_field_name("parameters") {
        c.get_children_by_kind("formal_parameter")
    } else {
        Vec::new()
    }
}

pub fn add_standalone_prefix(result: &mut String, shape: &Shape, context: &FmtContext) {
    if shape.standalone {
        result.push_str(&shape.indent.to_string(context.config));
    }
}

pub fn add_standalone_suffix(result: &mut String, shape: &Shape) {
    if shape.standalone {
        result.push_str(";");
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
