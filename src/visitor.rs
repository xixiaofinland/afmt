use crate::context::Context;
use crate::node_struct::{Class, NodeKind, Rewrite};
use crate::shape::Shape;
use anyhow::{anyhow, Result};
use tree_sitter::Node;

pub struct Visitor<'code> {
    pub context: Context<'code>,
}

impl<'code> Visitor<'code> {
    pub fn new(context: Context<'code>) -> Self {
        Self { context }
    }

    pub fn walk_root(&mut self, node: &Node, shape: &Shape) -> Result<String> {
        let mut results = Vec::new();

        let mut cursor = node.walk();
        let shape = Shape::default();
        for child in node.children(&mut cursor) {
            let kind = NodeKind::from_kind(child.kind());

            match kind {
                NodeKind::ClassDeclaration => {
                    let c = Class::new(&child, &shape);
                    results.push(self.visit_class(&c)?);
                }
                NodeKind::MethodDeclaration => {
                    //self.visit_method_node(node);
                }
                NodeKind::IfStatement => {
                    //self.visit_if_node(node);
                }
                NodeKind::ForLoop => {
                    //self.visit_for_node(node);
                }
                NodeKind::Unknown => !unimplemented!(),
            }
        }

        Ok(results.join(""))
    }

    pub fn visit_class(&mut self, c: &Class) -> Result<String> {
        let a = c
            .rewrite(&self.context)
            .ok_or_else(|| anyhow!("Format Class node failed!"))?;
        Ok(a)
    }
}
