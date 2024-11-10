use crate::data_model::*;
use crate::doc::{pretty_print, PrettyConfig};
use crate::doc_builder::DocBuilder;
use crate::utility::{collect_comments, enrich, set_thread_source_code};
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
    pub max_width: u32,

    #[serde(default = "default_indent_size")]
    pub indent_size: u32,
}

fn default_max_width() -> u32 {
    20
}

fn default_indent_size() -> u32 {
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
    pub fn new(max_width: u32) -> Self {
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

    pub fn max_width(&self) -> u32 {
        self.max_width
    }

    pub fn indent_size(&self) -> u32 {
        self.indent_size
    }
}

#[derive(Clone, Debug)]
pub struct Formatter {
    config: Config,
    source_files: Vec<String>,
    //pub errors: ReportedErrors,
}

impl Formatter {
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
    ) -> Result<Formatter> {
        let config = match config_path {
            Some(path) => Config::from_file(path)
                .map_err(|e| anyhow!(format!("{}: {}", e.to_string().yellow(), path)))?,
            None => Config::default(),
        };
        Ok(Formatter::new(config, source_files))
    }

    pub fn format(&self) -> Vec<Result<String>> {
        let (tx, rx) = mpsc::channel();
        let config = self.config.clone();

        for file in &self.source_files {
            let tx = tx.clone();
            let config = config.clone();
            let file = file.clone();

            thread::spawn(move || {
                let result = std::panic::catch_unwind(|| {
                    let source_code = fs::read_to_string(Path::new(&file))
                        .map_err(|e| {
                            anyhow!(format!(
                                "Failed to read file: {} {}",
                                &file.red(),
                                e.to_string().yellow()
                            ))
                        })
                        .unwrap();

                    Formatter::format_one(source_code, config)
                });
                match result {
                    Ok(result) => {
                        tx.send(Ok(result)).expect("failed to send result in tx");
                    }
                    Err(_) => tx
                        .send(Err(anyhow!("Thread panicked")))
                        .expect("failed to send error in tx"),
                }
            });
        }

        drop(tx);

        rx.into_iter().collect()
    }

    pub fn format_one(source_code: String, config: Config) -> String {
        let ast_tree = Formatter::parse(&source_code);

        set_thread_source_code(source_code); // important to set thread level source code now;

        // traverse the tree to collect all comment nodes
        let mut cursor = ast_tree.walk();
        let mut comments = Vec::new();
        collect_comments(&mut cursor, &mut comments);

        // traverse the tree to build enriched data
        let root: Root = enrich(&ast_tree);

        // Serialize to JSON with pretty printing
        //let serialized = serde_json::to_string_pretty(&root).unwrap();
        //println!("JSON: \n{}", serialized);

        // traverse enriched data and create pretty print combinators
        let c = PrettyConfig::new(config.indent_size);
        let b = DocBuilder::new(c);
        let doc_ref = root.build(&b);

        //pretty print
        let result = pretty_print(doc_ref, config.max_width);
        result
    }

    pub fn parse(source_code: &str) -> Tree {
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

        ast_tree
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
