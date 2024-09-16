use crate::{context::FmtContext, node_ext::*, shape::Shape, visitor::visit_node};
use anyhow::{bail, Context, Result};
use tree_sitter::Node;

use crate::shape::Indent;

pub fn try_add_standalone_prefix(result: &mut String, shape: &Shape, context: &FmtContext) {
    if shape.standalone {
        add_indent(result, shape, context);
    }
}

pub fn try_add_standalone_suffix(result: &mut String, shape: &Shape) {
    if shape.standalone {
        result.push(';');
    }
}

pub fn add_indent(result: &mut String, shape: &Shape, context: &FmtContext) {
    result.push_str(&shape.indent.as_string(context.config));
}

pub fn try_visit_cs(node: &Node, context: &FmtContext, shape: &mut Shape) -> Vec<String> {
    let mut cursor = node.walk();
    node.named_children(&mut cursor)
        .map(|n| visit_node(&n, context, shape))
        .collect::<Vec<_>>()
}

//pub fn get_indent_string(indent: &Indent) -> String {
//    let indent = "  ".repeat(indent.block_indent);
//    indent
//}

//pub fn is_standalone<'tree>(node: &Node<'tree>) -> bool {
//    if !node.is_named() {
//        return false;
//    }
//
//    if is_standalone_node(node) {
//        return true;
//    }
//
//    if let Some(parent) = node.parent() {
//        match parent.kind() {
//            "class_body" | "block" => true,
//            //"if_statement" => {
//            //    matches!(node.kind(), "block" | "if_statement") // For 'else if' and 'else' blocks
//            //}
//            _ => false,
//        }
//    } else {
//        !unreachable!() // all nodes should have a parent;
//    }
//}
//
//pub fn is_standalone_node<'tree>(node: &Node<'tree>) -> bool {
//    FULL_LINE_STATEMENTS.contains(&node.kind())
//}
//
//const FULL_LINE_STATEMENTS: &[&str] = &[
//    "class_declaration",
//    "method_declaration",
//    "if_statement",
//    "for_statement",
//    "while_statement",
//    "return_statement",
//    "local_variable_declaration",
//];
//
//pub fn has_body_node<'tree>(node: &Node<'tree>) -> bool {
//    HAS_BODY_NODE.contains(&node.kind())
//}
//const HAS_BODY_NODE: &[&str] = &[
//    "class_declaration",
//    "method_declaration",
//    "if_statement",
//    "for_statement",
//    "while_statement",
//];
