use crate::context::FmtContext;
use crate::shape::Shape;
use crate::visit::Visitor;
#[allow(unused_imports)]
use log::debug;
use tree_sitter::Node;

pub fn visit_root(context: &FmtContext) -> String {
    let mut result = String::new();
    let shape = Shape::empty(context.config);
    let root = &context.ast_tree.root_node();

    let mut cursor = root.walk();
    let children = root
        .named_children(&mut cursor)
        .map(|child| -> _ {
            let mut child_shape = shape.clone_with_stand_alone(true);
            child.visit(context, &mut child_shape)
        })
        .collect::<Vec<_>>()
        .join("\n");

    result.push_str(&children);

    // remove the extra "\n" introduced by the top-level class declaration
    result.truncate(result.trim_end_matches('\n').len());
    result
}

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
        if let Some(_) = node.next_named_sibling() {
            let count_new_lines = newlines_to_add(node, source_code);
            result.push_str(&"\n".repeat(count_new_lines));
        }
    }
}

pub fn try_add_standalone_suffix_no_semicolumn(
    node: &Node,
    result: &mut String,
    shape: &Shape,
    source_code: &str,
) {
    if shape.standalone {
        if let Some(_) = node.next_named_sibling() {
            let count_new_lines = newlines_to_add(node, source_code);
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
    let mut bytes_iter = remaining_code.bytes();

    match (bytes_iter.next(), bytes_iter.next()) {
        (Some(b'\n'), Some(b'\n')) => 1, // Two consecutive newlines
        _ => 0,                          // No or only one newline
    }
}
