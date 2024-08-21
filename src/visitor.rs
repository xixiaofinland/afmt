use crate::node_struct::{Class, NodeKind, Rewrite};
use crate::utility::NodeUtilities;
use anyhow::{anyhow, Result};
use tree_sitter::{Node, Tree, TreeCursor};

pub struct Visitor<'a> {
    pub formatted: String,
    pub block_indent: String,
    pub indent_level: usize,
    pub context: Context,
    pub root_node: &'a Node<'a>,
}

pub struct Context {
    config: String,
    parent_indent: String,
}

impl Context {
    pub fn new() -> Self {
        Context {
            config: String::new(),
            parent_indent: String::new(),
        }
    }
}

impl<'a> Visitor<'a> {
    pub fn new(root_node: &'a Node) -> Self {
        Visitor {
            formatted: String::new(),
            block_indent: String::from(' '),
            indent_level: 0,
            context: Context::new(),
            root_node,
        }
    }

    //https://github.com/dangmai/prettier-plugin-apex/blob/60db6549a441911a0ef25b0ecc5e61727dc92fbb/packages/prettier-plugin-apex/src/printer.ts#L612
    pub fn walk_from_root(&mut self) -> Result<()> {
        let mut cursor = self.root_node.walk();

        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();

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

                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }
        Ok(())
    }

    pub fn visit_class(&mut self, c: &Class) -> Result<()> {
        let a = c
            .rewrite()
            .ok_or_else(|| anyhow!("Format Class node failed!"))?;
        self.push_str(&a);
        Ok(())
    }

    pub fn get_formatted(&mut self) -> String {
        self.formatted.clone()
    }

    fn push_str(&mut self, s: &str) {
        self.formatted.push_str(s);
    }
}
