use crate::child::Accessor;
use crate::context::{language, FmtContext};
use crate::rewrite::Rewrite;
use crate::shape::Shape;
use crate::struct_def::{FromNode, IfStatement};
use crate::visit::Visitor;
use colored::Colorize;
#[allow(unused_imports)]
use log::debug;
use tree_sitter::{Node, Parser, Tree};

pub fn visit_root(context: &FmtContext) -> String {
    let mut result = String::new();
    let shape = Shape::empty(context.config);
    let root = &context.ast_tree.root_node();

    let mut cursor = root.walk();
    let children = root
        .named_children(&mut cursor)
        .map(|child| -> _ {
            let mut child_shape = shape.clone_with_standalone(true);
            child._visit(&mut child_shape, context)
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
        add_standalone_prefix(result, shape, context);
    }
}

pub fn add_standalone_prefix(result: &mut String, shape: &Shape, context: &FmtContext) {
    add_indent(result, shape, context);
}

pub fn try_add_standalone_suffix(
    node: &Node,
    result: &mut String,
    shape: &Shape,
    source_code: &str,
) {
    if shape.standalone {
        add_standalone_suffix(node, result, source_code);
    }
}

pub fn add_standalone_suffix(node: &Node, result: &mut String, source_code: &str) {
    result.push(';');
    if node.next_named_sibling().is_some() {
        let count_new_lines = newlines_to_add(node, source_code);
        result.push_str(&"\n".repeat(count_new_lines));
    }
}

pub fn try_add_standalone_suffix_no_semicolumn(
    node: &Node,
    result: &mut String,
    shape: &Shape,
    source_code: &str,
) {
    if shape.standalone && node.next_named_sibling().is_some() {
        add_standalone_suffix_no_semicolumn(node, result, source_code);
    }
}

pub fn add_standalone_suffix_no_semicolumn(node: &Node, result: &mut String, source_code: &str) {
    let count_new_lines = newlines_to_add(node, source_code);
    result.push_str(&"\n".repeat(count_new_lines));
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

pub fn rewrite<'a, 'tree, T>(n: &'a Node<'tree>, shape: &mut Shape, context: &FmtContext) -> String
where
    T: FromNode<'a, 'tree> + Rewrite,
{
    let block = T::new(n);
    block.rewrite(shape, context)
}

pub fn rewrite_shape<'a, 'tree, T>(
    n: &'a Node<'tree>,
    shape: &mut Shape,
    is_standalone: bool,
    context: &FmtContext,
) -> String
where
    T: FromNode<'a, 'tree> + Rewrite,
{
    let block = T::new(n);
    let cloned = &mut shape.clone_with_standalone(is_standalone);
    block.rewrite(cloned, context)
}

pub fn update_node<'a, 'tree>(node: &'a Node<'tree>, source_code: &str) -> (Tree, String) {
    let node_code = node.v(source_code);
    let mut new_node_code = String::from(node_code);

    if node.c_by_n("consequence").kind() != "block" {
        let consequence_code = node.c_by_n("consequence").v(node_code);
        new_node_code =
            new_node_code.replace(&consequence_code, &format!("{{ {} }}", consequence_code));
    }

    if let Some(ref a) = node.try_c_by_n("alternative") {
        if a.kind() != "block" {
            let alternative_code = a.v(source_code);
            new_node_code =
                new_node_code.replace(&alternative_code, &format!("{{ {} }}", alternative_code));
        }
    }

    let start_byte = node.start_byte();
    let end_byte = node.end_byte();

    let mut new_source_code = String::from(source_code);
    new_source_code.replace_range(start_byte..end_byte, &new_node_code);

    (reformat(&new_node_code), new_source_code)
}

fn reformat<'a, 'tree>(node_source_code: &str) -> Tree {
    println!("############## reformat_if_statement ########");
    let mut parser = Parser::new();
    parser
        .set_language(&language())
        .expect("Error loading Apex grammar when reformat if_statement.");

    // Apex parser can't parse if_statement alone
    let wrapped_source = format!("class Dummy {{ {{ {} }} }}", node_source_code);

    parser
        .parse(wrapped_source, None)
        .expect("parse updated if_statement failed in reformat().")
}
