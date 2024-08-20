use crate::node::{Class, NodeKind, Rewrite};
use anyhow::{anyhow, Result};
use tree_sitter::{Node, Tree};

pub struct Visitor {
    pub formatted: String,
    pub block_indent: String,
    pub indent_level: usize,
    pub context: Context,
    //pub node: &'a Node<'a>,
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

impl Visitor {
    pub fn new() -> Self {
        Visitor {
            formatted: String::new(),
            block_indent: String::from(' '),
            indent_level: 0,
            context: Context::new(),
        }
    }

    //https://github.com/dangmai/prettier-plugin-apex/blob/60db6549a441911a0ef25b0ecc5e61727dc92fbb/packages/prettier-plugin-apex/src/printer.ts#L612
    pub fn walk(&mut self, tree: &Tree) {
        let mut cursor = tree.walk();
        if cursor.goto_first_child() {
            loop {
                let node = &cursor.node();

                let kind = NodeKind::from_kind(node.kind());

                match kind {
                    NodeKind::ClassDeclaration => {
                        let c = Class::new(&node);
                        self.visit_class(&c);
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
