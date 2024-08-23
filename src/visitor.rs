use crate::node_struct::{Class, NodeKind, Rewrite};
use anyhow::{anyhow, Result};
use tree_sitter::Node;

#[derive(Default)]
pub struct Visitor {}

impl Visitor {
    //https://github.com/dangmai/prettier-plugin-apex/blob/60db6549a441911a0ef25b0ecc5e61727dc92fbb/packages/prettier-plugin-apex/src/printer.ts#L612
    pub fn walk(&mut self, node: &Node) -> Result<String> {
        let mut result = String::new();

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            let kind = NodeKind::from_kind(child.kind());

            match kind {
                NodeKind::ClassDeclaration => {
                    let c = Class::new(&child);
                    self.visit_class(&c)?;
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
        Ok(result)
    }

    pub fn visit_class(&mut self, c: &Class) -> Result<String> {
        let a = c
            .rewrite()
            .ok_or_else(|| anyhow!("Format Class node failed!"))?;
        Ok(a)
    }
}
