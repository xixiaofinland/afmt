use crate::context::FmtContext;
use anyhow::Result;
use std::sync::{mpsc, Arc};
use std::thread;
use std::{fs, path::Path};

#[derive(Clone)]
pub struct Config {
    pub max_width: usize,
    pub indent_size: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_width: 80,
            indent_size: 2,
        }
    }
}

impl Config {
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
        let (tx, rx) = mpsc::channel();
        let config = Arc::new(self.config.clone());

        for file in &self.source_files {
            let tx = tx.clone();
            let config = Arc::clone(&config);
            let file = file.clone();

            thread::spawn(move || {
                let source_code =
                    fs::read_to_string(Path::new(&file)).expect("Failed to read file");

                let context = FmtContext::new(&config, &source_code);
                let result = context.format_one_file();
                tx.send(result).expect("failed to send result in tx");
            });
        }

        drop(tx);

        rx.into_iter().collect()
    }
}
