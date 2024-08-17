use afmt;
use tree_sitter::{Node, Parser}; // Replace with the correct module path

fn main() {
    let mut parser = Parser::new();
    parser
        .set_language(&afmt::language())
        .expect("Error loading Apex grammar");

    let code = "public class MyClass {
        public void hello() {
        true;
        }
    }";

    let mut tree = parser.parse(code, None).unwrap();
    let root_node = tree.root_node();
    if root_node.has_error() {
        println!("root node found error!");
        return;
    }

    traverse(root_node);
}

fn traverse(node: Node) {
    if node.child_count() == 0 {
        println!("Leaf node: {}", node.kind());
        println!("Leaf node: {}", node.start_position());
        if node.has_error() {
            println!("error ndoe: {}", node.kind());
        }
    } else {
        for i in 0..node.child_count() {
            let child = node.child(i).unwrap();
            println!("child node: {}", node.kind());
            if node.has_error() {
                println!("error ndoe: {} ", node.kind());
            }
            traverse(child);
        }
    }
}
