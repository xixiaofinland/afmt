use tree_sitter::Node;

use crate::utility::{get_source_code, panic_missing_mandatory_child};

// `c` => child
// `cv` => child value
// `cs` => children
// `csv` => children value
// `by_n` => by name
// `by_k` => by kind
#[allow(dead_code)]
pub trait Accessor<'t> {
    fn value(&self) -> String;

    fn first_c(&self) -> Node<'t>;

    fn try_first_c(&self) -> Option<Node<'t>>;
    fn try_c_by_n(&self, kind: &str) -> Option<Node<'t>>;
    fn try_c_by_k(&self, kind: &str) -> Option<Node<'t>>;
    fn try_cs_by_k(&self, kind: &str) -> Vec<Node<'t>>;

    fn c_by_n(&self, name: &str) -> Node<'t>;
    fn c_by_k(&self, kind: &str) -> Node<'t>;
    fn cvalue_by_n(&self, name: &str) -> String;
    fn cvalue_by_k(&self, name: &str) -> String;

    fn children_vec(&self) -> Vec<Node<'t>>;
    fn cs_by_k(&self, kind: &str) -> Vec<Node<'t>>;
    fn cs_by_n(&self, name: &str) -> Vec<Node<'t>>;

    fn next_named(&self) -> Node<'t>;

    // private fn;
    fn v<'a>(&self) -> &'a str;
    fn cv_by_k(&self, name: &str) -> &str;
    fn cv_by_n<'a>(&self, name: &str) -> &'a str;
}

impl<'t> Accessor<'t> for Node<'t> {
    fn next_named(&self) -> Node<'t> {
        let mut sibling = self.next_named_sibling();
        while let Some(node) = sibling {
            if !node.is_extra() {
                return node;
            }
            sibling = node.next_named_sibling();
        }
        panic!("## {}: next_named node missing.", self.kind());
    }

    fn v<'a>(&self) -> &'a str {
        self.utf8_text(get_source_code().as_bytes())
            .unwrap_or_else(|_| panic!("## {}: get source_code value failed.", self.kind()))
    }

    fn value(&self) -> String {
        self.v().to_string()
    }

    fn children_vec(&self) -> Vec<Node<'t>> {
        let mut cursor = self.walk();
        self.named_children(&mut cursor)
            .filter(|node| !node.is_extra())
            .collect()
    }

    fn try_c_by_k(&self, kind: &str) -> Option<Node<'t>> {
        let mut cursor = self.walk();
        let child = self.named_children(&mut cursor).find(|c| c.kind() == kind);
        child
    }

    fn try_cs_by_k(&self, kind: &str) -> Vec<Node<'t>> {
        let mut cursor = self.walk();
        self.named_children(&mut cursor)
            .filter(|c| c.kind() == kind)
            .collect()
    }

    fn try_c_by_n(&self, name: &str) -> Option<Node<'t>> {
        self.child_by_field_name(name)
    }

    fn c_by_k(&self, kind: &str) -> Node<'t> {
        self.try_c_by_k(kind)
            .unwrap_or_else(|| panic_missing_mandatory_child(self, kind))
    }

    fn try_first_c(&self) -> Option<Node<'t>> {
        let mut index = 0;
        while let Some(node) = self.named_child(index) {
            if !node.is_extra() {
                return Some(node);
            }
            index += 1;
        }
        None
    }

    fn first_c(&self) -> Node<'t> {
        let mut index = 0;
        while let Some(node) = self.named_child(index) {
            if !node.is_extra() {
                return node;
            }
            index += 1;
        }
        panic!(
            "## {}: missing a mandatory child in first_c().",
            self.kind()
        );
    }

    fn cv_by_k(&self, name: &str) -> &str {
        let child_node = self.c_by_k(name);
        child_node.v()
    }

    fn cv_by_n<'a>(&self, name: &str) -> &'a str {
        let node = self
            .child_by_field_name(name)
            .unwrap_or_else(|| panic_missing_mandatory_child(self, name));
        node.v()
    }

    fn cvalue_by_n(&self, name: &str) -> String {
        self.cv_by_n(name).to_string()
    }

    fn cvalue_by_k(&self, name: &str) -> String {
        self.cv_by_k(name).to_string()
    }

    fn c_by_n(&self, name: &str) -> Node<'t> {
        self.child_by_field_name(name)
            .unwrap_or_else(|| panic_missing_mandatory_child(self, name))
    }

    fn cs_by_n(&self, name: &str) -> Vec<Node<'t>> {
        let mut cursor = self.walk();
        let children: Vec<Node<'t>> = self.children_by_field_name(name, &mut cursor).collect();
        if children.is_empty() {
            panic_missing_mandatory_child(self, name)
        }
        children
    }

    fn cs_by_k(&self, kind: &str) -> Vec<Node<'t>> {
        let children = self.try_cs_by_k(kind);
        if children.is_empty() {
            panic_missing_mandatory_child(self, kind)
        }
        children
    }
}
