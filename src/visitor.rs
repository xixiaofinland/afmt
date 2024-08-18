use tree_sitter::Node;

pub struct Visitor<'a> {
    pub buffer: String,
    pub block_indent: String,
    pub node: &'a Node<'a>,
    pub indent_level: usize,
}

impl<'a> Visitor<'a> {
    pub fn init(node: &'a Node) -> Self {
        Visitor {
            buffer: String::new(),
            block_indent: String::from(' '),
            node,
            indent_level: 0,
        }
    }

    pub fn visit_node(&mut self) {
        println!("visit_class_node: node kind: {}", self.node.kind());
        self.push_str("formatted");
    }

    pub fn get_formatted(&self) -> String {
        self.buffer.clone()
    }

    fn push_str(&mut self, s: &str) {
        self.buffer.push_str(s);
    }
}
