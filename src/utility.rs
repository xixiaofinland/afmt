use anyhow::{bail, Context, Result};
use tree_sitter::Node;

pub trait NodeUtilities<'tree> {
    fn get_child_by_kind(&self, kind: &str) -> Option<Node<'tree>>;
    fn get_children_by_kind(&self, kind: &str) -> Vec<Node<'tree>>;
}

impl<'tree> NodeUtilities<'tree> for Node<'tree> {
    fn get_child_by_kind(&self, kind: &str) -> Option<Node<'tree>> {
        let mut cursor = self.walk();
        for child in self.children(&mut cursor) {
            if child.kind() == kind {
                return Some(child);
            }
        }
        None
    }

    fn get_children_by_kind(&self, kind: &str) -> Vec<Node<'tree>> {
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
