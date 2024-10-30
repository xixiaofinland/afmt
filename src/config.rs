use crate::utility::{collect_comments, enrich};
use anyhow::{anyhow, Result};
use colored::Colorize;
use serde::Deserialize;
use std::sync::{mpsc, Arc};
use std::thread;
use std::{fs, path::Path};
use tree_sitter::{Language, Node, Parser, Tree};

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_max_width")]
    pub max_width: usize,

    #[serde(default = "default_indent_size")]
    pub indent_size: usize,
}

fn default_max_width() -> usize {
    80
}

fn default_indent_size() -> usize {
    2
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_width: default_max_width(),
            indent_size: default_indent_size(),
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

    pub fn from_file(path: &str) -> Result<Self> {
        let content =
            fs::read_to_string(path).map_err(|e| anyhow!("Failed to read config file: {}", e))?;
        let config: Config =
            toml::from_str(&content).map_err(|e| anyhow!("Failed to parse config file: {}", e))?;
        Ok(config)
    }

    pub fn max_width(&self) -> usize {
        self.max_width
    }

    pub fn indent_size(&self) -> usize {
        self.indent_size
    }
}

#[derive(Clone, Debug)]
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

    pub fn create_from_config(
        config_path: Option<&str>,
        source_files: Vec<String>,
    ) -> Result<Session> {
        let config = match config_path {
            Some(path) => Config::from_file(path)
                .map_err(|e| anyhow!(format!("{}: {}", e.to_string().yellow(), path)))?,
            None => Config::default(),
        };
        Ok(Session::new(config, source_files))
    }

    pub fn format(&self) {
        let file = &self.source_files[0];
        let source_code = fs::read_to_string(Path::new(file))
            .map_err(|e| {
                anyhow!(format!(
                    "Failed to read file: {} {}",
                    &file.red(),
                    e.to_string().yellow()
                ))
            })
            .unwrap();

        let context = FmtContext::new(&self.config, source_code);

        // traverse the tree to collect all comment nodes
        let mut cursor = context.ast_tree.walk();
        let mut comments = Vec::new();
        collect_comments(&mut cursor, &mut comments, &context);

        // traverse the tree to build enriched data

        // traverse enriched data and create combinators to print result

        let _ = enrich(&context);

        //let (tx, rx) = mpsc::channel();
        //let config = Arc::new(self.config.clone());
        //
        //for file in &self.source_files {
        //    let tx = tx.clone();
        //    let config = Arc::clone(&config);
        //    let file = file.clone();
        //
        //    thread::spawn(move || {
        //        let result = std::panic::catch_unwind(|| {
        //            let source_code = fs::read_to_string(Path::new(&file)).map_err(|e| {
        //                anyhow!(format!(
        //                    "Failed to read file: {} {}",
        //                    &file.red(),
        //                    e.to_string().yellow()
        //                ))
        //            })?;
        //            let context = FmtContext::new(&config, source_code);
        //            context.format_one_file()
        //        });
        //        match result {
        //            Ok(result) => tx.send(result).expect("failed to send result in tx"),
        //            Err(_) => tx
        //                .send(Err(anyhow!("Thread panicked")))
        //                .expect("failed to send error in tx"),
        //        }
        //    });
        //}
        //
        //drop(tx);
        //
        //rx.into_iter().collect()
    }
}

#[derive(Clone)]
pub struct FmtContext<'a> {
    pub config: &'a Config,
    pub source_code: String,
    pub ast_tree: Tree,
}

impl<'a> FmtContext<'a> {
    pub fn new(config: &'a Config, source_code: String) -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(&language())
            .expect("Error loading Apex grammar");

        let ast_tree = parser.parse(&source_code, None).unwrap();
        let root_node = &ast_tree.root_node();

        if root_node.has_error() {
            if let Some(error_node) = Self::find_last_error_node(root_node) {
                let error_snippet = &source_code[error_node.start_byte()..error_node.end_byte()];
                println!(
                    "Error in node kind: {}, at byte range: {}-{}, snippet: {}",
                    error_node.kind().yellow(),
                    error_node.start_byte(),
                    error_node.end_byte(),
                    error_snippet,
                );
                if let Some(p) = error_node.parent() {
                    let parent_snippet = &source_code[p.start_byte()..p.end_byte()];
                    println!(
                        "Parent node kind: {}, at byte range: {}-{}, snippet: {}",
                        p.kind().yellow(),
                        p.start_byte(),
                        p.end_byte(),
                        parent_snippet,
                    );
                }
            }
            panic!("{}", "Parser encounters an error node in the tree.".red());
        }

        Self {
            config,
            source_code,
            ast_tree,
        }
    }

    fn find_last_error_node<'tree>(node: &Node<'tree>) -> Option<Node<'tree>> {
        if !node.has_error() {
            return None; // If the current node has no error, return None
        }

        let mut last_error_node = Some(*node);

        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                if child.has_error() {
                    last_error_node = Self::find_last_error_node(&child);
                }
            }
        }

        last_error_node // Return the last (deepest) error node
    }
}

extern "C" {
    fn tree_sitter_apex() -> Language;
}

pub fn language() -> Language {
    unsafe { tree_sitter_apex() }
}
