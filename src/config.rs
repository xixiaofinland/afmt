use anyhow::Result;
use std::{fs, path::Path};
use tree_sitter::{Language, Parser};

use crate::visitor::Visitor;

#[derive(Default, Clone)]
pub struct Shape {
    pub block_indent: usize,
}

impl Shape {
    pub fn new(block_indent: usize) -> Self {
        Self { block_indent }
    }
}

#[derive(Debug)]
pub struct Args {
    pub path: String,
}

#[derive(Clone)]
pub struct Config {
    pub max_line_len: usize,
}

impl Config {
    pub fn new(max_line_len: usize) -> Self {
        Self { max_line_len }
    }
}

pub struct Context<'a> {
    pub config: Config,
    pub source_code: &'a str,
}

impl<'a> Context<'a> {
    pub fn new(config: Config, source_code: &'a str) -> Self {
        Self {
            config,
            source_code,
        }
    }

    pub fn format(&self) -> Result<String> {
        let mut parser = Parser::new();
        parser
            .set_language(&language())
            .expect("Error loading Apex grammar");

        let tree = parser.parse(self.source_code, None).unwrap();
        let root_node = tree.root_node();
        if root_node.has_error() {
            panic!("Parsing with errors in the tree.")
        }

        let shape = Shape::default();
        let visitor = Visitor::default();
        let mut result = visitor.walk(&root_node, self, &shape)?;

        // add file ending new line;
        result.push('\n');

        Ok(result)
    }
}

pub struct Session {
    pub config: Config,
    source_files: Vec<String>,
    //pub errors: ReportedErrors,
}

impl Session {
    pub fn new(config: Config, source_files: Vec<String>) -> Self {
        Self {
            config,
            source_files,
            //errors: ReportedErrors::default(),
        }
    }

    pub fn format(&self) -> Vec<Result<String>> {
        self.source_files
            .iter()
            .map(|f| {
                let config = self.config.clone();
                let source_code = fs::read_to_string(Path::new(f)).expect("Failed to read file");
                let context = Context::new(config, &source_code);
                context.format()
            })
            .collect()
    }
}

extern "C" {
    fn tree_sitter_apex() -> Language;
}

pub fn language() -> Language {
    unsafe { tree_sitter_apex() }
}
