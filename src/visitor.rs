use crate::node_struct::*;
use crate::shape::Shape;
use anyhow::{anyhow, Result};
use tree_sitter::Node;

pub fn walk(node: &Node, parent_shape: &Shape) -> Option<String> {
    let mut results = Vec::new();

    let is_root_node = node.kind() == "parser_output";

    let shape = if is_root_node {
        Shape::new(0)
    } else {
        Shape::new(parent_shape.block_indent + 1)
    };

    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        let kind = NodeKind::from_kind(child.kind());

        match kind {
            NodeKind::ClassDeclaration => {
                let c = ClassDeclaration::new(&child, &shape);
                results.push(c.rewrite()?);
            }
            NodeKind::FieldDeclaration => {
                let f = FieldDeclaration::new(&child, &shape);
                results.push(f.rewrite()?);
            }
            //NodeKind::MethodDeclaration => {
            //    //self.visit_method_node(node);
            //}
            //NodeKind::IfStatement => {
            //    //self.visit_if_node(node);
            //}
            //NodeKind::ForLoop => {
            //    //self.visit_for_node(node);
            //}
            NodeKind::Unknown => {
                println!("### Unknow node: {}", child.kind());
                !unimplemented!();
            }
        }
    }

    Some(results.join(""))
}
