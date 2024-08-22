use anyhow::{bail, Context, Result};
use tree_sitter::Node;

pub trait NodeUtilities {
    fn get_child_by_kind(&self, kind: &str) -> Option<Node>;
    fn get_children_by_kind(&self, kind: &str) -> Vec<Node>;
}

impl NodeUtilities for Node<'_> {
    fn get_child_by_kind(&self, kind: &str) -> Option<Node> {
        let mut cursor = self.walk();
        for child in self.children(&mut cursor) {
            if child.kind() == kind {
                return Some(child);
            }
        }
        None
    }

    fn get_children_by_kind(&self, kind: &str) -> Vec<Node> {
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
