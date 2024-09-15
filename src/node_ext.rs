use anyhow::{bail, Context, Result};
use tree_sitter::{Node, TreeCursor};

pub trait NodeExt<'tree> {
    fn v<'a>(&self, source_code: &'a str) -> &'a str;

    fn try_c_by_n(&self, kind: &str) -> Option<Node<'tree>>;

    fn try_cv_by_n<'a>(&self, name: &str, source_code: &'a str) -> Option<&'a str>;

    fn try_c_by_k(&self, kind: &str) -> Option<Node<'tree>>;
    fn try_cv_by_k<'a>(&self, kind: &str, source_code: &'a str) -> Option<&'a str>;

    fn try_cs_by_k(&self, kind: &str) -> Vec<Node<'tree>>;

    fn c_by_k(&self, kind: &str) -> Node<'tree>;
    fn cv_by_k<'a>(&self, name: &str, source_code: &'a str) -> &'a str;

    fn cs_by_k(&self, kind: &str) -> Vec<Node<'tree>>;

    fn c_by_n(&self, name: &str) -> Node<'tree>;
    fn cv_by_n<'a>(&self, name: &str, source_code: &'a str) -> &'a str;

    fn try_csv_by_k<'a>(&self, kind: &str, source_code: &'a str) -> Vec<&'a str>;

    fn get_children_by_name(&self, name: &str) -> Vec<Node<'tree>>;
}

impl<'tree> NodeExt<'tree> for Node<'tree> {
    fn v<'a>(&self, source_code: &'a str) -> &'a str {
        self.utf8_text(source_code.as_bytes())
            .unwrap_or_else(|_| panic!("{}: get_value failed.", self.kind()))
    }

    fn try_c_by_k(&self, kind: &str) -> Option<Node<'tree>> {
        let mut cursor = self.walk();
        let child = self.children(&mut cursor).find(|c| c.kind() == kind);
        child
    }

    fn try_cs_by_k(&self, kind: &str) -> Vec<Node<'tree>> {
        let mut cursor = self.walk();
        self.children(&mut cursor)
            .filter(|c| c.kind() == kind)
            .collect()
    }

    fn try_c_by_n(&self, name: &str) -> Option<Node<'tree>> {
        self.child_by_field_name(name)
    }

    fn try_cv_by_n<'a>(&self, name: &str, source_code: &'a str) -> Option<&'a str> {
        self.child_by_field_name(name).map(|n| n.v(source_code))
    }

    fn c_by_k(&self, kind: &str) -> Node<'tree> {
        self.try_c_by_k(kind)
            .unwrap_or_else(|| panic!("mandatory kind child: {} not found.", kind))
    }

    fn cv_by_k<'a>(&self, name: &str, source_code: &'a str) -> &'a str {
        let child_node = self.c_by_k(name);
        child_node.v(source_code)
    }

    fn cv_by_n<'a>(&self, name: &str, source_code: &'a str) -> &'a str {
        let node = self
            .child_by_field_name(name)
            .unwrap_or_else(|| panic!("mandatory named child: {} missing.", name));
        node.v(source_code)
    }

    fn c_by_n(&self, name: &str) -> Node<'tree> {
        self.child_by_field_name(name)
            .unwrap_or_else(|| panic!("mandatory named child: {} missing.", name))
    }

    fn get_children_by_name(&self, name: &str) -> Vec<Node<'tree>> {
        let mut cursor = self.walk();
        let children: Vec<Node<'tree>> = self.children_by_field_name(name, &mut cursor).collect();
        if children.is_empty() {
            panic!("Mandatory named children: {} missing", name);
        }
        children
    }

    fn cs_by_k(&self, kind: &str) -> Vec<Node<'tree>> {
        let children = self.try_cs_by_k(kind);
        if children.is_empty() {
            panic!("Mandatory kind children: {} missing", kind);
        }
        children
    }

    fn try_csv_by_k<'a>(&self, kind: &str, source_code: &'a str) -> Vec<&'a str> {
        self.try_cs_by_k(kind)
            .iter()
            .map(|n| n.v(source_code))
            .collect::<Vec<&str>>()
    }

    fn try_cv_by_k<'a>(&self, kind: &str, source_code: &'a str) -> Option<&'a str> {
        self.try_c_by_k(kind).map(|child| child.v(source_code))
    }
}
