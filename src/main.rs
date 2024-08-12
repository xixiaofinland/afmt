use afmt;
use tree_sitter::{Language, Parser}; // Replace with the correct module path

fn main() {
    let mut parser = Parser::new();
    parser
        .set_language(&afmt::language())
        .expect("Error loading Apex grammar");

    let code = r#"
    public class MyClass {
        public void myMethod() {
            System.debug('Hello, world!');
        }
    }
    "#;

    let tree = parser.parse(code, None).unwrap();
    println!("{:?}", tree.root_node().to_sexp());
}
