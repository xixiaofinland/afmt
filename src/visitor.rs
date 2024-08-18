use tree_sitter::{Node, Tree};

pub struct Visitor {
    pub formatted: String,
    pub block_indent: String,
    pub indent_level: usize,
    //pub node: &'a Node<'a>,
}

impl Visitor {
    pub fn init() -> Self {
        Visitor {
            formatted: String::new(),
            block_indent: String::from(' '),
            indent_level: 0,
        }
    }

    pub fn walk(&mut self, tree: &Tree) {
        let mut cursor = tree.walk();
        if cursor.goto_first_child() {
            loop {
                let node = &cursor.node();

                match node.kind() {
                    "class_declaration" => {
                        self.visit_class_node(node);
                        self.push_str("class_declaration formatting visited");
                    }

                    _ => {
                        unimplemented!()
                    }
                }

                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }
    }

    pub fn get_formatted(&mut self) -> String {
        self.formatted.clone()
    }

    pub fn visit_class_node(&mut self, node: &Node) {
        self.push_str("visit_class_node called");
        println!("visit_class_node: node kind: {}", node.kind());
    }

    //pub fn get_formatted(&self) -> String {
    //    self.buffer.clone()
    //}

    fn push_str(&mut self, s: &str) {
        self.formatted.push_str(s);
    }
}
