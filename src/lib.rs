mod context;
mod extension;
mod macros;
mod node_struct;
mod shape;
mod utility;
mod visitor;

use shape::Shape;
use tree_sitter::Language;
use tree_sitter::{Node, Parser};
use utility::*;
use visitor::walk;

extern "C" {
    fn tree_sitter_apex() -> Language;
}

pub fn language() -> Language {
    unsafe { tree_sitter_apex() }
}

pub fn set_context_and_format_code() -> Option<String> {
    let source_code = get_source_code_from_arg();
    format_code(&source_code)
}

pub fn format_code(source_code: &str) -> Option<String> {
    set_global_context(source_code.to_string());

    let mut parser = Parser::new();
    parser
        .set_language(&language())
        .expect("Error loading Apex grammar");

    let tree = parser.parse(source_code, None).unwrap();
    let root_node = tree.root_node();
    if root_node.has_error() {
        panic!("Parsing with errors in the tree.")
    }

    let shape = Shape::default();
    let mut result = walk(&root_node, &shape)?;

    // add file ending new line;
    result.push('\n');

    Some(result)
}

/// The content of the [`node-types.json`][] file for this grammar.
//pub const NODE_TYPES: &str = include_str!("node-types.json");

// Uncomment these to include any queries that this grammar contains
// pub const HIGHLIGHTS_QUERY: &str = include_str!("../../queries/highlights.scm");
// pub const INJECTIONS_QUERY: &str = include_str!("../../queries/injections.scm");
// pub const LOCALS_QUERY: &str = include_str!("../../queries/locals.scm");
// pub const TAGS_QUERY: &str = include_str!("../../queries/tags.scm");

#[cfg(test)]
mod tests {
    #[test]
    fn test_can_load_grammar() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&super::language())
            .expect("Error loading Apex grammar");
    }
}
