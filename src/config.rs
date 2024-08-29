use crate::context::Context;
use anyhow::Result;
use std::{fs, path::Path};

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
