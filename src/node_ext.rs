use anyhow::{bail, Context, Result};
use tree_sitter::Node;

pub trait NodeExt<'tree> {
    fn get_value<'a>(&self, source_code: &'a str) -> &'a str;

    fn get_child_by_kind(&self, kind: &str) -> Option<Node<'tree>>;
    fn get_children_by_kind(&self, kind: &str) -> Vec<Node<'tree>>;

    fn get_mandatory_child_by_kind(&self, kind: &str) -> Node<'tree>;
    fn get_mandatory_child_value_by_kind<'a>(&self, name: &str, source_code: &'a str) -> &'a str;
    fn get_mandatory_children_by_kind(&self, kind: &str) -> Vec<Node<'tree>>;

    fn get_mandatory_child_by_name(&self, name: &str) -> Node<'tree>;
    fn get_mandatory_children_by_name(&self, name: &str) -> Vec<Node<'tree>>;
    fn get_mandatory_child_value_by_name<'a>(&self, name: &str, source_code: &'a str) -> &'a str;

    fn get_modifiers_value(&self, source_code: &str) -> String;
}

impl<'tree> NodeExt<'tree> for Node<'tree> {
    fn get_value<'a>(&self, source_code: &'a str) -> &'a str {
        self.utf8_text(source_code.as_bytes())
            .expect(&format!("{}: get_value failed.", self.kind()))
    }

    fn get_child_by_kind(&self, kind: &str) -> Option<Node<'tree>> {
        let mut cursor = self.walk();
        let child = self.children(&mut cursor).find(|c| c.kind() == kind);
        child
    }

    fn get_children_by_kind(&self, kind: &str) -> Vec<Node<'tree>> {
        let mut cursor = self.walk();
        self.children(&mut cursor)
            .filter(|c| c.kind() == kind)
            .collect()
    }

    fn get_mandatory_child_by_kind(&self, kind: &str) -> Node<'tree> {
        self.get_child_by_kind(kind)
            .unwrap_or_else(|| panic!("mandatory kind child: {} not found.", kind))
    }

    fn get_mandatory_child_value_by_kind<'a>(&self, name: &str, source_code: &'a str) -> &'a str {
        let child_node = self.get_mandatory_child_by_kind(name);
        child_node.get_value(source_code)
    }

    fn get_mandatory_child_value_by_name<'a>(&self, name: &str, source_code: &'a str) -> &'a str {
        let node = self
            .child_by_field_name(name)
            .unwrap_or_else(|| panic!("mandatory named child: {} missing.", name));
        node.get_value(source_code)
    }

    fn get_mandatory_child_by_name(&self, name: &str) -> Node<'tree> {
        self.child_by_field_name(name)
            .unwrap_or_else(|| panic!("mandatory named child: {} missing.", name))
    }

    fn get_mandatory_children_by_name(&self, name: &str) -> Vec<Node<'tree>> {
        let mut cursor = self.walk();
        let children: Vec<Node<'tree>> = self.children_by_field_name(name, &mut cursor).collect();
        if children.is_empty() {
            panic!("Mandatory named children: {} missing", name);
        }
        children
    }

    fn get_mandatory_children_by_kind(&self, kind: &str) -> Vec<Node<'tree>> {
        let children = self.get_children_by_kind(kind);
        if children.is_empty() {
            panic!("Mandatory kind children: {} missing", kind);
        }
        children
    }

    fn get_modifiers_value(&self, source_code: &str) -> String {
        let modifier_nodes = self.get_children_by_kind("modifier");
        modifier_nodes
            .iter()
            .map(|n| n.get_value(source_code))
            .collect::<Vec<&str>>()
            .join(" ")
    }
}
