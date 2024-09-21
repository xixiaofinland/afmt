use colored::Colorize;
use tree_sitter::Node;

// `c` => child
// `cv` => child value
// `cs` => children
// `csv` => children value
// `by_n` => by name
// `by_k` => by kind
#[allow(dead_code)]
pub trait Accessor<'tree> {
    fn v<'a>(&self, source_code: &'a str) -> &'a str;

    fn try_c_by_n(&self, kind: &str) -> Option<Node<'tree>>;
    fn try_c_by_k(&self, kind: &str) -> Option<Node<'tree>>;
    fn try_cv_by_n<'a>(&self, name: &str, source_code: &'a str) -> Option<&'a str>;
    fn try_cv_by_k<'a>(&self, kind: &str, source_code: &'a str) -> Option<&'a str>;
    fn try_cs_by_k(&self, kind: &str) -> Vec<Node<'tree>>;
    fn try_csv_by_k<'a>(&self, kind: &str, source_code: &'a str) -> Vec<&'a str>;

    fn c_by_n(&self, name: &str) -> Node<'tree>;
    fn c_by_k(&self, kind: &str) -> Node<'tree>;
    fn first_c(&self) -> Node<'tree>;
    fn cv_by_k<'a>(&self, name: &str, source_code: &'a str) -> &'a str;
    fn cv_by_n<'a>(&self, name: &str, source_code: &'a str) -> &'a str;
    fn cs_by_k(&self, kind: &str) -> Vec<Node<'tree>>;
    fn cs_by_n(&self, name: &str) -> Vec<Node<'tree>>;
}

impl<'tree> Accessor<'tree> for Node<'tree> {
    fn v<'a>(&self, source_code: &'a str) -> &'a str {
        self.utf8_text(source_code.as_bytes())
            .expect(&format!("{}: get_value failed.", self.kind().red()))
    }

    fn try_c_by_k(&self, kind: &str) -> Option<Node<'tree>> {
        let mut cursor = self.walk();
        let child = self.named_children(&mut cursor).find(|c| c.kind() == kind);
        child
    }

    fn try_cs_by_k(&self, kind: &str) -> Vec<Node<'tree>> {
        let mut cursor = self.walk();
        self.named_children(&mut cursor)
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
        self.try_c_by_k(kind).expect(&format!(
            "{}: missing mandatory kind child: {}.",
            self.kind().red(),
            kind.red()
        ))
    }

    fn first_c(&self) -> Node<'tree> {
        self.named_child(0).expect(&format!(
            "{}: missing a mandatory child.",
            self.kind().red()
        ))
    }

    fn cv_by_k<'a>(&self, name: &str, source_code: &'a str) -> &'a str {
        let child_node = self.c_by_k(name);
        child_node.v(source_code)
    }

    fn cv_by_n<'a>(&self, name: &str, source_code: &'a str) -> &'a str {
        let node = self.child_by_field_name(name).expect(&format!(
            "{}: missing mandatory name child: {}.",
            self.kind().red(),
            name.red()
        ));
        node.v(source_code)
    }

    fn c_by_n(&self, name: &str) -> Node<'tree> {
        self.child_by_field_name(name).expect(&format!(
            "{}: missing mandatory name child: {}.",
            self.kind().red(),
            name.red()
        ))
    }

    fn cs_by_n(&self, name: &str) -> Vec<Node<'tree>> {
        let mut cursor = self.walk();
        let children: Vec<Node<'tree>> = self.children_by_field_name(name, &mut cursor).collect();
        if children.is_empty() {
            panic!(
                "{}: missing mandatory name child: {}.",
                self.kind().red(),
                name.red()
            );
        }
        children
    }

    fn cs_by_k(&self, kind: &str) -> Vec<Node<'tree>> {
        let children = self.try_cs_by_k(kind);
        if children.is_empty() {
            panic!(
                "{}: missing mandatory kind children: {}.",
                self.kind().red(),
                kind.red()
            );
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
