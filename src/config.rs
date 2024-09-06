use crate::context::FmtContext;
use anyhow::Result;
use std::{fs, path::Path};

#[derive(Clone)]
pub struct Config {
    pub max_width: usize,
}

impl Config {
    pub fn default() -> Self {
        Self { max_width: 80 }
    }

    pub fn new(max_width: usize) -> Self {
        Self { max_width }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Indent {
    pub block_indent: usize,
    pub alignment: usize,
}

impl Indent {
    pub fn default() -> Indent {
        Indent::new(0, 0)
    }

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
                let context = FmtContext::new(&self.config, &source_code);
                context.format_one_file()
            })
            .collect()
    }
}
