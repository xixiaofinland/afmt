use crate::context::FmtContext;
use anyhow::Result;
use std::{fs, path::Path};

#[derive(Clone)]
pub struct Config {
    max_width: usize,
    indent_size: usize,
}

impl Config {
    pub fn default() -> Self {
        Self {
            max_width: 80,
            indent_size: 2,
        }
    }

    pub fn new(max_width: usize) -> Self {
        Self {
            max_width,
            indent_size: 2,
        }
    }

    pub fn max_width(&self) -> usize {
        self.max_width
    }

    pub fn indent_size(&self) -> usize {
        self.indent_size
    }
}

pub struct Session {
    config: Config,
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

    pub fn config(&self) -> &Config {
        &self.config
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
