use crate::{config::Config, shape::Shape, visitor::Visitor};
use anyhow::Result;
use tree_sitter::{Language, Parser, Tree};

#[derive(Clone)]
pub struct FmtContext<'a> {
    pub config: &'a Config,
    pub source_code: &'a str,
    pub ast_tree: Tree,
}

impl<'a> FmtContext<'a> {
    pub fn new(config: &'a Config, source_code: &'a str) -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(&language())
            .expect("Error loading Apex grammar");

        let ast_tree = parser.parse(source_code, None).unwrap();
        let root_node = &ast_tree.root_node();
        if root_node.has_error() {
            panic!("Parsing with errors in the tree.")
        }

        Self {
            config,
            source_code,
            ast_tree,
        }
    }

    pub fn format_one_file(&self) -> Result<String> {
        let mut result = String::new();

        let mut visitor = Visitor::default();
        result.push_str(&visitor.visit_root(self));
        // add file ending new line;
        result.push('\n');

        Ok(result)
    }
}

extern "C" {
    fn tree_sitter_apex() -> Language;
}

pub fn language() -> Language {
    unsafe { tree_sitter_apex() }
}
