use anyhow::{bail, Context, Result};
use tree_sitter::Node;

pub trait NodeUtilities {
    fn get_child_by_kind(&self);
    fn get_children_by_kind(&self);
}

impl NodeUtilities for Node {
    pub fn get_child_by_kind<'a>(&self, kind: &str) -> Option<Node<'a>> {
        let mut cursor = self.walk();
        for child in self.children(&mut cursor) {
            if child.kind() == kind {
                return Some(child);
            }
        }
        None
    }

    pub fn get_children_by_kind<'a>(&self, kind: &str) -> Vec<Node<'a>> {
        let mut cursor = self.walk();
        let mut modifiers = Vec::new();
        for child in self.children(&mut cursor) {
            if child.kind() == kind {
                modifiers.push(child);
            }
        }
        modifiers
    }
}
