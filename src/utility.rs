use log::debug;
use tree_sitter::Node;

use crate::{context::FmtContext, shape::Shape};

pub fn try_add_standalone_prefix(result: &mut String, shape: &Shape, context: &FmtContext) {
    if shape.standalone {
        add_indent(result, shape, context);
    }
}

pub fn try_add_standalone_suffix(
    node: &Node,
    result: &mut String,
    shape: &Shape,
    source_code: &str,
) {
    if shape.standalone {
        result.push(';');
        if let Some(n) = node.next_named_sibling() {
            debug!("node: {}", n.kind());
            let count_new_lines = newlines_to_add(node, source_code);
            debug!("new_line: {}", count_new_lines);
            result.push_str(&"\n".repeat(count_new_lines));
        }
    }
}

pub fn add_indent(result: &mut String, shape: &Shape, context: &FmtContext) {
    result.push_str(&shape.indent.as_string(context.config));
}

fn newlines_to_add(node: &Node, source_code: &str) -> usize {
    let index = node.end_byte();
    if index >= source_code.len() {
        return 0;
    }

    let remaining_code = &source_code[index..];
    debug!("code: {}", remaining_code);
    let mut bytes_iter = remaining_code.bytes();

    match (bytes_iter.next(), bytes_iter.next()) {
        (Some(b'\n'), Some(b'\n')) => 1, // Two consecutive newlines
        _ => 0,                          // No or only one newline
    }
}
