use anyhow::{bail, Result};
use std::{fs, path::Path};
use tree_sitter::{Language, Node, Parser};

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

#[derive(Clone)]
pub struct Config {
    pub max_line_len: usize,
}

impl Config {
    pub fn new(max_line_len: usize) -> Self {
        Self { max_line_len }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Indent {
    pub block_indent: usize,
    pub alignment: usize,
}

impl Indent {
    pub fn new(block_indent: usize, alignment: usize) -> Indent {
        Indent {
            block_indent,
            alignment,
        }
    }
}

#[derive(Clone)]
pub struct Context<'a> {
    pub config: &'a Config,
    pub source_code: &'a str,
}

impl<'a> Context<'a> {
    pub fn new(config: &'a Config, source_code: &'a str) -> Self {
        Self {
            config,
            source_code,
        }
    }

    pub fn format_one_file(&self) -> Result<String> {
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
        let mut visitor = Visitor::new(None, Indent::new(0, 0));
        let mut result = visitor.traverse(&root_node, self, &shape)?;

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
                //let config = self.config.clone();
                let source_code = fs::read_to_string(Path::new(f)).expect("Failed to read file");
                let context = Context::new(&self.config, &source_code);
                context.format_one_file()
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
