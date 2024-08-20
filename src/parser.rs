use anyhow::{bail, Context, Result};
use tree_sitter::Node;

pub fn run_it(node: &Node) -> Result<()> {
    //let count = node.named_child_count();
    //count
    //println!("node kind: {}", node.kind());
    //
    //let n = node.child_by_field_name("modifiers").context("no!")?;
    //println!("r: {}", n.kind());
    //Ok(())

    println!("node kind: {}", node.kind());

    for i in 0..node.child_count() {
        let child = node.child(i).unwrap();
        let field_name = node.field_name_for_child(i.try_into().unwrap());
        println!(
            "Child {}: kind: {}, field name: {:?}, range: {:?}",
            i,
            child.kind(),
            field_name,
            child.range()
        );
    }

    let n = node.child_by_field_name("modifiers").context("no!")?;
    println!("r: {}", n.kind());
    Ok(())
}
